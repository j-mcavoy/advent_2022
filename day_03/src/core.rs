mod compartment;
mod item;
pub mod rucksack;
pub type Answer = u32;
use rucksack::Rucksack;

pub fn str2rucksacks(s: &str) -> Vec<Rucksack> {
    s.lines().map(|l| l.into()).collect()
}
