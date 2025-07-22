use fyrox_core::ImmutableString;

pub struct NamedValuesContainerRef<'a, T> {
    properties: &'a [NamedValue<T>],
}

impl<T> NamedValuesContainerRef<'_, T> {
    pub fn property_ref(&self, name: &ImmutableString) -> Option<&NamedValue<T>> {
        search(self.properties, name)
    }
}

pub struct NamedValuesContainer<T, const N: usize> {
    properties: [NamedValue<T>; N],
}

fn search<'a, T>(slice: &'a [NamedValue<T>], name: &ImmutableString) -> Option<&'a NamedValue<T>> {
    slice
        .binary_search_by(|prop| prop.name.cached_hash().cmp(&name.cached_hash()))
        .ok()
        .and_then(|idx| slice.get(idx))
}

impl<T, const N: usize> NamedValuesContainer<T, N> {
    pub fn property_ref(&self, name: &ImmutableString) -> Option<&NamedValue<T>> {
        search(&self.properties, name)
    }

    pub fn data_ref(&self) -> NamedValuesContainerRef<'_, T> {
        NamedValuesContainerRef {
            properties: &self.properties,
        }
    }
}

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
