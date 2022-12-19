use super::*;

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl TryFrom<char> for Item {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_alphabetic() {
            Ok(Self(value))
        } else {
            Err("Not ASCII alpha character".into())
        }
    }
}
