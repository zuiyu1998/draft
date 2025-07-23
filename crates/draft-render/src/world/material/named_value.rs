use fyrox_core::ImmutableString;

pub struct NamedValue<T> {
    pub name: ImmutableString,
    pub value: T,
}

impl<T> NamedValue<T> {
    pub fn new(name: impl Into<ImmutableString>, value: T) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}
