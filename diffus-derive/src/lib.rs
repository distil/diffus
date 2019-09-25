extern crate proc_macro;

use quote::{
    quote,
    format_ident,
};

type Output = proc_macro2::TokenStream;

// FIXME verify support for field name `other` et al. ex. struct A { other: u32 }
// havoc
// I think its okey now but setup regression test

// FIXME namespacing or something to avoid collisions in what is generated
//
// FIXME organize all the small helpers edit_field et al
//
// FIXME possible to avoid EditedX to pollute namespace, have it associated? X::Edited is that
// possible?
//
// FIXME think about how we want to organize the namespacing of all things ex.
// enm::Edit::VariantChange

fn edit_fields(
    fields: &syn::Fields,
) -> Output {
    let edit_fields = fields.iter()
        .map(|field| {
            match field {
                syn::Field { ident: Some(ident), ty, .. } => quote! {
                    #ident: diffus::edit::Edit<'a, #ty>
                },
                        syn::Field { ident: None, ty, .. } => quote! {
                    diffus::edit::Edit<'a, #ty>
                },
            }
        });

    quote! { #(#edit_fields),* }
}

fn field_ident(
    enumerated_field: (usize, &syn::Field),
    prefix: &str,
) -> syn::Ident {
    match enumerated_field {
        (_, syn::Field { ident: Some(ident), .. }) => {
            format_ident!("{}{}", prefix, ident)
        }
        (i, syn::Field { ident: None, .. }) => {
            format_ident!("{}{}", prefix, unnamed_field_ident(i))
        }
    }
}

fn field_idents(
    fields: &syn::Fields,
    prefix: &str,
) -> Output {
    let field_idents = fields.iter().enumerate()
        .map(|enumerated_field| field_ident(enumerated_field, prefix));

    quote! { #(#field_idents),* }
}

fn renamed_field_ident(
    enumerated_field: (usize, &syn::Field),
    prefix: &str,
) -> Output {
    match enumerated_field {
        (_, syn::Field { ident: Some(ident), .. }) => {
            let new_ident = format_ident!("{}{}", prefix, ident);

            quote! { #ident: #new_ident }
        }
        (_, syn::Field { ident: None, .. }) => unreachable!(),
    }
}

fn renamed_field_idents(
    fields: &syn::Fields,
    prefix: &str,
) -> Output {
    let field_idents = fields.iter().enumerate()
        .map(|enumerated_field| renamed_field_ident(enumerated_field, prefix));

    quote! { #(#field_idents),* }
}

fn matches_all_copy(
    fields: &syn::Fields
) -> Output {
    let edit_fields_copy = fields.iter().enumerate()
        .map(|enumerated_field| {
            let field_ident = field_ident(enumerated_field, "");

            quote! { #field_ident @ diffus::edit::Edit::Copy }
        });

    quote! {
        ( #(#edit_fields_copy),* ) => diffus::edit::Edit::Copy
    }
}

fn field_diffs(
    fields: &syn::Fields
) -> Output {
    let field_diffs = fields.iter()
        .enumerate()
        .map(|(index, field)| {
            let field_name = match field {
                syn::Field { ident: Some(ident), .. } => quote! { #ident },
                syn::Field { ident: None, .. } => {
                    let ident = unnamed_field_name(index);

                    quote! { #ident }
                }
            };

            quote! {
                self.#field_name.diff(&other.#field_name)
            }
        });

    quote! { #(#field_diffs),* }
}


fn unnamed_field_ident(
    i: usize,
) -> syn::Ident {
    format_ident!("x{}", i as u32)
}
fn unnamed_field_name(
    i: usize,
) -> syn::Lit {
    syn::parse_str(&format!("{}", i as u32)).unwrap()
}


#[proc_macro_derive(Diffus)]
pub fn derive_diffus(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse2(proc_macro2::TokenStream::from(input)).unwrap();

    let ident = &input.ident;
    let vis = &input.vis;
    let edited_ident = syn::parse_str::<syn::Path>(&format!("Edited{}", ident)).unwrap();

    match input.data {
        syn::Data::Enum(syn::DataEnum {
            variants,
            ..
        }) => {
            let edit_variants = variants.iter().map(|syn::Variant {
                ident, fields, ..
            }| {
                let edit_fields = edit_fields(&fields);

                match fields {
                    syn::Fields::Named(syn::FieldsNamed { .. }) => {
                        quote! {
                            #ident { #edit_fields }
                        }
                    },
                    syn::Fields::Unnamed(syn::FieldsUnnamed { .. }) => {
                        quote! {
                            #ident ( #edit_fields )
                        }
                    },
                    syn::Fields::Unit => {
                        quote! {
                            #ident
                        }
                    },
                }

            });

            let variants_matches = variants.iter().map(|syn::Variant { ident: variant_ident, fields, .. }| {

                let field_diffs = fields.iter().enumerate().map(|(i, field)| {
                    let self_field_ident = field_ident((i, field), "self_");
                    let other_field_ident = field_ident((i, field), "other_");

                    quote! {
                        #self_field_ident . diff(& #other_field_ident )
                    }
                });
                let field_diffs = quote! { #(#field_diffs),* };

                let matches_all_copy = matches_all_copy(&fields);
                let just_field_idents = field_idents(&fields, "");
                let self_field_idents = field_idents(&fields, "self_");
                let other_field_idents = field_idents(&fields, "other_");

                match fields {
                    syn::Fields::Named(syn::FieldsNamed { .. }) => {
                        let self_field_idents = renamed_field_idents(&fields, "self_");
                        let other_field_idents = renamed_field_idents(&fields, "other_");

                        quote! {
                            (
                                #ident::#variant_ident { #self_field_idents },
                                #ident::#variant_ident { #other_field_idents }
                            ) => {
                                match ( #field_diffs ) {
                                    #matches_all_copy,
                                    ( #just_field_idents ) => {
                                        diffus::edit::Edit::Change(
                                            diffus::edit::enm::Edit::AssociatedChanged(
                                                #edited_ident::#variant_ident { #just_field_idents }
                                            )
                                        )
                                    }
                                }
                            }
                        }
                    },
                    syn::Fields::Unnamed(syn::FieldsUnnamed { .. }) => {
                        quote! {
                            (
                                #ident::#variant_ident( #self_field_idents ),
                                #ident::#variant_ident( #other_field_idents )
                            ) => {
                                match ( #field_diffs ) {
                                    #matches_all_copy,
                                    ( #just_field_idents ) => {
                                        diffus::edit::Edit::Change(
                                            diffus::edit::enm::Edit::AssociatedChanged(
                                                #edited_ident::#variant_ident ( #just_field_idents )
                                            )
                                        )
                                    }
                                }
                            }
                        }
                    },
                    syn::Fields::Unit => {
                        quote! {
                            (
                                #ident::#variant_ident,
                                #ident::#variant_ident
                            ) => {
                                diffus::edit::Edit::Copy
                            }
                        }
                    },
                }
            });

            proc_macro::TokenStream::from(quote! {
                #vis enum #edited_ident<'a> {
                    #(#edit_variants),*
                }

                impl<'a> diffus::Diffable<'a> for #ident {
                    type D = diffus::edit::enm::Edit<'a, Self, #edited_ident<'a>>;

                    fn diff(&'a self, other: &'a Self) -> diffus::edit::Edit<'a, Self> {
                        match (self, other) {
                            #(#variants_matches,)*
                            (self_variant, other_variant) => diffus::edit::Edit::Change(diffus::edit::enm::Edit::VariantChanged(
                                self_variant, other_variant
                            )),
                        }
                    }
                }

            })
        },
        syn::Data::Struct(syn::DataStruct {
            fields,
            ..
        }) => {
            let edit_fields = edit_fields(&fields);
            let field_diffs = field_diffs(&fields);
            let field_idents = field_idents(&fields, "");
            let matches_all_copy = matches_all_copy(&fields);

            match fields {
                syn::Fields::Named(_) => {
                    proc_macro::TokenStream::from(quote! {
                        #vis struct #edited_ident<'a> {
                            #edit_fields
                        }

                        impl<'a> diffus::Diffable<'a> for #ident {
                            type D = #edited_ident<'a>;

                            fn diff(&'a self, other: &'a Self) -> diffus::edit::Edit<'a, Self> {
                                match ( #field_diffs ) {
                                    #matches_all_copy,
                                    ( #field_idents ) => diffus::edit::Edit::Change(
                                        #edited_ident { #field_idents }
                                    )
                                }
                            }
                        }
                    })
                },
                syn::Fields::Unnamed(_) => {
                    proc_macro::TokenStream::from(quote! {
                        #vis struct #edited_ident<'a> ( #edit_fields );

                        impl<'a> diffus::Diffable<'a> for #ident {
                            type D = #edited_ident<'a>;

                            fn diff(&'a self, other: &'a Self) -> diffus::edit::Edit<'a, Self> {
                                match ( #field_diffs ) {
                                    #matches_all_copy,
                                    ( #field_idents ) => diffus::edit::Edit::Change(
                                        #edited_ident ( #field_idents )
                                    )
                                }
                            }
                        }
                        
                    })
                },
                syn::Fields::Unit => {
                    proc_macro::TokenStream::from(quote! {
                        #vis struct #edited_ident;

                        impl<'a> diffus::Diffable<'a> for #ident {
                            type D = #edited_ident;

                            fn diff(&'a self, other: &'a Self) -> diffus::edit::Edit<'a, Self> {
                                diffus::edit::Edit::Copy
                            }
                        }
                    })
                }
            }
        },
        syn::Data::Union(_) => panic!("union type not supported"),
    }
}
