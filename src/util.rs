pub enum BoO<'a, T> {
    Borrowed(&'a T),
    Owned(Box<T>),
}

impl<'a, T> BoO<'a, T> {
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

impl<'a, T> From<T> for BoO<'a, T> {
    fn from(t: T) -> Self {
        Self::Owned(Box::new(t))
    }
}

impl<'a, T> From<&'a T> for BoO<'a, T> {
    fn from(t: &'a T) -> Self {
        Self::Borrowed(t)
    }
}

impl<'a, T> AsRef<T> for BoO<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            BoO::Borrowed(t) => *t,
            BoO::Owned(t) => t,
        }
    }
}

impl<'a, T> std::ops::Deref for BoO<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
