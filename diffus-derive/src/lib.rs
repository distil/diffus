extern crate proc_macro;

use quote::{format_ident, quote};

type Output = proc_macro2::TokenStream;

fn edit_fields(fields: &syn::Fields, lifetime: &syn::Lifetime) -> Output {
    let edit_fields = fields.iter().map(|field| match field {
        syn::Field {
            ident: Some(ident),
            ty,
            vis,
            ..
        } => quote! {
            #vis #ident: diffus::edit::Edit<#lifetime, #ty>
        },
        syn::Field {
            ident: None,
            ty,
            vis,
            ..
        } => quote! {
            #vis diffus::edit::Edit<#lifetime, #ty>
        },
    });

    quote! { #(#edit_fields),* }
}

fn field_ident(enumerated_field: (usize, &syn::Field), prefix: &str) -> syn::Ident {
    match enumerated_field {
        (
            _,
            syn::Field {
                ident: Some(ident), ..
            },
        ) => format_ident!("{}{}", prefix, ident),
        (i, syn::Field { ident: None, .. }) => {
            format_ident!("{}{}", prefix, unnamed_field_ident(i))
        }
    }
}

fn field_idents(fields: &syn::Fields, prefix: &str) -> Output {
    let field_idents = fields
        .iter()
        .enumerate()
        .map(|enumerated_field| field_ident(enumerated_field, prefix));

    quote! { #(#field_idents),* }
}

fn renamed_field_ident(enumerated_field: (usize, &syn::Field), prefix: &str) -> Output {
    match enumerated_field {
        (
            _,
            syn::Field {
                ident: Some(ident), ..
            },
        ) => {
            let new_ident = format_ident!("{}{}", prefix, ident);

            quote! { #ident: #new_ident }
        }
        (_, syn::Field { ident: None, .. }) => unreachable!(),
    }
}

fn renamed_field_idents(fields: &syn::Fields, prefix: &str) -> Output {
    let field_idents = fields
        .iter()
        .enumerate()
        .map(|enumerated_field| renamed_field_ident(enumerated_field, prefix));

    quote! { #(#field_idents),* }
}

fn matches_all_copy(fields: &syn::Fields) -> Output {
    let edit_fields_copy = fields.iter().enumerate().map(|_| {
        quote! { diffus::edit::Edit::Copy(_) }
    });

    quote! {
        ( #(#edit_fields_copy),* ) => diffus::edit::Edit::Copy(self)
    }
}

fn field_diffs(fields: &syn::Fields) -> Output {
    let field_diffs = fields.iter().enumerate().map(|(index, field)| {
        let field_name = match field {
            syn::Field {
                ident: Some(ident), ..
            } => quote! { #ident },
            syn::Field { ident: None, .. } => {
                let ident = unnamed_field_name(index);

                quote! { #ident }
            }
        };

        quote! {
            diffus::Diffable::diff(&self.#field_name, &other.#field_name)
        }
    });

    quote! { #(#field_diffs),* }
}

fn unnamed_field_ident(i: usize) -> syn::Ident {
    format_ident!("x{}", i as u32)
}
fn unnamed_field_name(i: usize) -> syn::Lit {
    syn::parse_str(&format!("{}", i as u32)).unwrap()
}

fn input_lifetime(generics: &syn::Generics) -> Option<&syn::Lifetime> {
    let mut lifetimes = generics.params.iter().filter_map(|generic_param| {
        if let syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime, .. }) = generic_param {
            Some(lifetime)
        } else {
            None
        }
    });

    let lifetime = lifetimes.next();

    assert!(
        lifetimes.next().is_none(),
        "Multiple lifetimes not supported yet"
    );

    lifetime
}

struct Generics {
    ty_generic_params: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,

    edited_ty_generic_params: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    edited_ty_where_clause: syn::WhereClause,

    impl_diffable_generic_params: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    impl_diffable_where_clause: syn::WhereClause,

    impl_lifetime: syn::Lifetime,
}

#[proc_macro_derive(Diffus)]
pub fn derive_diffus(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse2(proc_macro2::TokenStream::from(input)).unwrap();

    let ident = &input.ident;
    let vis = &input.vis;

    let edited_ident = syn::parse_str::<syn::Path>(&format!("Edited{}", ident)).unwrap();

    let Generics {
        ty_generic_params,
        edited_ty_generic_params, edited_ty_where_clause,
        impl_diffable_generic_params, impl_diffable_where_clause,
        impl_lifetime,
    } = Generics::new(&input.generics, &input.data);

    #[cfg(feature = "serialize-impl")]
    let derive_serialize = Some(quote! { #[derive(serde::Serialize)] });
    #[cfg(not(feature = "serialize-impl"))]
    let derive_serialize: Option<proc_macro2::TokenStream> = None;

    proc_macro::TokenStream::from(match input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let edit_variants = variants.iter().map(|syn::Variant { ident, fields, .. }| {
                let edit_fields = edit_fields(&fields, &impl_lifetime);

                match fields {
                    syn::Fields::Named(syn::FieldsNamed { .. }) => {
                        quote! {
                            #ident { #edit_fields }
                        }
                    }
                    syn::Fields::Unnamed(syn::FieldsUnnamed { .. }) => {
                        quote! {
                            #ident ( #edit_fields )
                        }
                    }
                    syn::Fields::Unit => {
                        quote! {
                            #ident
                        }
                    }
                }
            });

            let has_non_unit_variant = variants.iter().any(|syn::Variant { fields, .. }| {
                if let syn::Fields::Unit = fields {
                    false
                } else {
                    true
                }
            });

            let unit_enum_generic_params = if has_non_unit_variant {
                Some(edited_ty_generic_params.clone())
            } else {
                None
            };

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
                                diffus::edit::Edit::Copy(self)
                            }
                        }
                    },
                }
            });

            quote! {
                #derive_serialize
                #vis enum #edited_ident <#unit_enum_generic_params> #edited_ty_where_clause {
                    #(#edit_variants),*
                }

                impl<#impl_diffable_generic_params> diffus::Diffable<#impl_lifetime> for #ident <#ty_generic_params> #impl_diffable_where_clause {
                    type Diff = diffus::edit::enm::Edit<#impl_lifetime, Self, #edited_ident <#unit_enum_generic_params>>;

                    fn diff(&#impl_lifetime self, other: &#impl_lifetime Self) -> diffus::edit::Edit<#impl_lifetime, Self> {
                        match (self, other) {
                            #(#variants_matches,)*
                            (self_variant, other_variant) => diffus::edit::Edit::Change(diffus::edit::enm::Edit::VariantChanged(
                                self_variant, other_variant
                            )),
                        }
                    }
                }
            }
        }
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            let edit_fields = edit_fields(&fields, &impl_lifetime);
            let field_diffs = field_diffs(&fields);
            let field_idents = field_idents(&fields, "");
            let matches_all_copy = matches_all_copy(&fields);

            match fields {
                syn::Fields::Named(_) => {
                    quote! {
                        #derive_serialize
                        #vis struct #edited_ident<#edited_ty_generic_params> #edited_ty_where_clause {
                            #edit_fields
                        }

                        impl<#impl_diffable_generic_params> diffus::Diffable<#impl_lifetime> for #ident <#ty_generic_params> #impl_diffable_where_clause {
                            type Diff = #edited_ident<#edited_ty_generic_params>;

                            fn diff(&#impl_lifetime self, other: &#impl_lifetime Self) -> diffus::edit::Edit<#impl_lifetime, Self> {
                                match ( #field_diffs ) {
                                    #matches_all_copy,
                                    ( #field_idents ) => diffus::edit::Edit::Change(
                                        #edited_ident { #field_idents }
                                    )
                                }
                            }
                        }
                    }
                }
                syn::Fields::Unnamed(_) => {
                    quote! {
                        #derive_serialize
                        #vis struct #edited_ident<#edited_ty_generic_params> ( #edit_fields ) #edited_ty_where_clause;

                        impl<#impl_diffable_generic_params> diffus::Diffable<#impl_lifetime> for #ident <#ty_generic_params> #impl_diffable_where_clause {
                            type Diff = #edited_ident<#edited_ty_generic_params>;

                            fn diff(&#impl_lifetime self, other: &#impl_lifetime Self) -> diffus::edit::Edit<#impl_lifetime, Self> {
                                match ( #field_diffs ) {
                                    #matches_all_copy,
                                    ( #field_idents ) => diffus::edit::Edit::Change(
                                        #edited_ident ( #field_idents )
                                    )
                                }
                            }
                        }
                    }
                }
                syn::Fields::Unit => {
                    quote! {
                        #derive_serialize
                        #vis struct #edited_ident< > #edited_ty_where_clause;

                        impl<#impl_lifetime> diffus::Diffable<#impl_lifetime> for #ident< > #impl_diffable_where_clause {
                            type Diff = #edited_ident;

                            fn diff(&#impl_lifetime self, other: &#impl_lifetime Self) -> diffus::edit::Edit<#impl_lifetime, Self> {
                                diffus::edit::Edit::Copy(self)
                            }
                        }
                    }
                }
            }
        }
        syn::Data::Union(_) => panic!("union type not supported yet"),
    })
}

impl Generics {
    pub fn new(
        input_generics: &syn::Generics,
        data: &syn::Data,
    ) -> Self {
        let input_generic_params = &input_generics.params;
        let input_where_clause = &input_generics.where_clause;
        let empty_where_clause = input_generics.clone().make_where_clause().clone();

        let generic_types_used = Self::collect_generic_types_used(input_generics, data);

        let ty_generic_params = input_generic_params.clone();

        let mut edited_ty_generic_params = ty_generic_params.clone();
        let mut edited_ty_where_clause = input_where_clause.clone().unwrap_or(empty_where_clause.clone());

        let mut impl_diffable_generic_params = input_generic_params.clone();
        let mut impl_diffable_where_clause = input_where_clause.clone().unwrap_or(empty_where_clause);

        let explicit_data_lifetime = input_lifetime(input_generics);
        let impl_lifetime = explicit_data_lifetime.cloned().unwrap_or_else(|| {
            let default_lifetime = syn::parse_str::<syn::Lifetime>("'diffus_a").unwrap();

            // Add the lifetime into the generics lists.
            impl_diffable_generic_params.insert(0, syn::GenericParam::Lifetime(syn::LifetimeDef::new(default_lifetime.clone())));
            edited_ty_generic_params.insert(0, syn::GenericParam::Lifetime(syn::LifetimeDef::new(default_lifetime.clone())));

            default_lifetime.clone()
        });

        // Ensure that all generic types that exist live for as long as the diffus lifetime.
        impl_diffable_where_clause.predicates.extend(input_generics.type_params().map(|type_param| {
            let where_predicate = quote!(#type_param : #impl_lifetime);
            let where_predicate: syn::WherePredicate = syn::parse(where_predicate.into()).unwrap();
            where_predicate
        }));

        // Ensure that all generic types actually used are diffable and live for as long as the
        // diffus lifetime.
        for generic_ty_path in generic_types_used {
            let where_predicate = quote!(#generic_ty_path : diffus::Diffable<#impl_lifetime> + #impl_lifetime);
            let where_predicate = syn::parse::<syn::WherePredicate>(where_predicate.into()).unwrap();

            impl_diffable_where_clause.predicates.push(where_predicate.clone());
            edited_ty_where_clause.predicates.push(where_predicate.clone());
        }

        Generics {
            ty_generic_params,
            edited_ty_generic_params, edited_ty_where_clause,
            impl_diffable_generic_params, impl_diffable_where_clause,
            impl_lifetime,
        }
    }

    /// Collects all of the generic types used in a type including associated types.
    fn collect_generic_types_used(
        input_generics: &syn::Generics,
        data: &syn::Data,
    ) -> Vec<syn::Path> {
        let all_possible_fields: Vec<&syn::Fields> = match *data {
            syn::Data::Struct(ref s) => vec![&s.fields],
            syn::Data::Enum(ref e) => e.variants.iter().map(|v| &v.fields).collect(),
            syn::Data::Union(..) => Vec::new(), // unimplemented
        };

        let all_possible_types: Vec<&syn::Type> = all_possible_fields.into_iter().flat_map(|fields| match fields {
            syn::Fields::Named(ref fields) => fields.named.iter().map(|f| &f.ty).collect(),
            syn::Fields::Unnamed(ref fields) => fields.unnamed.iter().map(|f| &f.ty).collect(),
            syn::Fields::Unit => Vec::new(),
        }).collect();

        let mut generic_types_used = Vec::new();
        let mut remaining_types_to_check = all_possible_types.clone();

        while let Some(type_to_check) = remaining_types_to_check.pop() {
            match *type_to_check {
                syn::Type::Path(ref path) => {
                    if let Some(first_segment) = path.path.segments.first().map(|s| &s.ident) {
                        let first_segment: syn::Ident = first_segment.clone().into();

                        if input_generics.type_params().any(|type_param| type_param.ident == first_segment) {
                            generic_types_used.push(path.path.clone());
                        }
                    }
                },

                syn::Type::Array(ref array) => remaining_types_to_check.push(&array.elem),
                syn::Type::Group(ref group) => remaining_types_to_check.push(&group.elem),
                syn::Type::Paren(ref paren) => remaining_types_to_check.push(&paren.elem),
                syn::Type::Ptr(ref ptr) => remaining_types_to_check.push(&ptr.elem),
                syn::Type::Reference(ref reference) => remaining_types_to_check.push(&reference.elem),
                syn::Type::Slice(ref slice) => remaining_types_to_check.push(&slice.elem),
                syn::Type::Tuple(ref tuple) => remaining_types_to_check.extend(tuple.elems.iter()),
                syn::Type::Verbatim(..) |
                    syn::Type::ImplTrait(..) |
                    syn::Type::Infer(..) |
                    syn::Type::Macro(..) |
                    syn::Type::Never(..) |
                    syn::Type::TraitObject(..) |
                    syn::Type::BareFn(..) => (),
                _ => (), // unknown/unsupported type
            }
        }

        generic_types_used
    }
}
