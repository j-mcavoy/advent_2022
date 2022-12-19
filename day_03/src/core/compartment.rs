use super::item::Item;
use std::collections::BTreeSet;

type Items = BTreeSet<Item>;
#[derive(Clone)]
pub struct Compartment(pub Items);

impl From<Items> for Compartment {
    fn from(value: Items) -> Self {
        Self(value)
    }
}
impl From<&str> for Compartment {
    fn from(s: &str) -> Self {
        s.chars()
            .map(|c| c.try_into().unwrap())
            .collect::<Items>()
            .into()
    }
}
