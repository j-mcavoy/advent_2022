mod traits;

#[derive(Clone, Copy, Eq)]
pub struct Item(char);

impl Item {
    pub fn value(&self) -> u8 {
        // A-Z (65-90)
        // a-z (97-122)
        u8::try_from(
            u8::try_from(self.0).unwrap() as i16 - 64
                + (if self.0.is_lowercase() { -32 } else { 26 }),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_value() {
        assert_eq!(1, Item::try_from('a').unwrap().value());
        assert_eq!(26, Item::try_from('z').unwrap().value());
        assert_eq!(27, Item::try_from('A').unwrap().value());
        assert_eq!(52, Item::try_from('Z').unwrap().value());
    }
}
