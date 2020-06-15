pub enum Boo<'a, T> {
    Borrowed(&'a T),
    Owned(Box<T>),
}

impl<'a, T> Boo<'a, T> {
    pub fn borrowed(self) -> Option<&'a T> {
        match self {
            Self::Owned(_) => None,
            Self::Borrowed(t) => Some(t),
        }
    }
    pub fn owned(self) -> Option<T> {
        match self {
            Self::Borrowed(_) => None,
            Self::Owned(t) => Some(*t),
        }
    }
}

impl<'a, T: Default> Boo<'a, T> {
    pub fn from_some_ref_or(opt: Option<&'a T>, owned: T) -> Self {
        match opt.as_ref() {
            Some(v) => Self::Borrowed(v),
            None => Self::Owned(Box::from(owned)),
        }
    }
    pub fn from_some_ref_or_else(opt: Option<&'a T>, t_fn: impl FnOnce() -> T) -> Self {
        match opt.as_ref() {
            Some(v) => Self::Borrowed(v),
            None => Self::Owned(Box::from(t_fn())),
        }
    }
    pub fn from_some_ref_or_default(opt: Option<&'a T>) -> Self {
        match opt.as_ref() {
            Some(v) => Self::Borrowed(v),
            None => Self::Owned(Box::from(T::default())),
        }
    }
}

impl<'a, T> From<T> for Boo<'a, T> {
    fn from(t: T) -> Self {
        Self::Owned(Box::new(t))
    }
}

impl<'a, T> From<&'a T> for Boo<'a, T> {
    fn from(t: &'a T) -> Self {
        Self::Borrowed(t)
    }
}

impl<'a, T> AsRef<T> for Boo<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            Boo::Borrowed(t) => *t,
            Boo::Owned(t) => t,
        }
    }
}

impl<'a, T> std::ops::Deref for Boo<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
