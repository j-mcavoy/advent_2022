use std::collections::BTreeSet;

use super::compartment::Compartment;
use super::item::Item;

#[derive(Clone)]
pub struct Rucksack(Compartment, Compartment);
impl Rucksack {
    pub fn common_item(&self) -> Item {
        let intersection: Vec<Item> = self.0 .0.intersection(&self.1 .0).cloned().collect();
        assert_eq!(intersection.len(), 1);
        *intersection.get(0).unwrap()
    }
    pub fn badge(r1: &Self, r2: &Self, r3: &Self) -> Item {
        let r1: BTreeSet<Item> = (r1.0 .0).union(&r1.1 .0).cloned().collect();
        let r2: BTreeSet<Item> = (r2.0 .0).union(&r2.1 .0).cloned().collect();
        let r3: BTreeSet<Item> = (r3.0 .0).union(&r3.1 .0).cloned().collect();

        let intersection: Vec<Item> = r1
            .intersection(&r2)
            .cloned()
            .collect::<BTreeSet<Item>>()
            .intersection(&r3)
            .cloned()
            .collect();
        assert_eq!(intersection.len(), 1);
        *intersection.get(0).unwrap()
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        assert!(s.len() % 2 == 0); // assert line is even
        let (lhs, rhs) = s.split_at(s.len() / 2);

        Rucksack(lhs.into(), rhs.into())
    }
}
