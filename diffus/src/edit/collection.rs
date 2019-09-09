pub enum Edit<T: Eq> {
    Copy(T),
    Add(T),
    Remove(T),
}

impl<T: Eq + std::fmt::Debug> std::fmt::Debug for Edit<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Copy(value) => write!(f, "Copy({:?})", value),
            Self::Add(value) => write!(f, "Add({:?})", value),
            Self::Remove(value) => write!(f, "Remove({:?})", value),
        }
    }
}

impl<T: Eq + PartialEq> PartialEq for Edit<T> {
    fn eq(&self, other: &Self) -> bool {
        let left = match self {
            Self::Copy(left) | Self::Add(left) | Self::Remove(left) => left,
        };
        let right = match other {
            Self::Copy(right) | Self::Add(right) | Self::Remove(right) => right,
        };
        left == right
    }
}

impl<T: Eq> Eq for Edit<T> {}
