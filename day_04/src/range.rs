use crate::id::*;

#[derive(PartialEq, Debug)]
pub struct Range(Id, Id);
impl Range {
    #[allow(dead_code)]
    pub fn new<T: TryInto<Id>>(lower: T, upper: T) -> Self {
        (lower, upper).try_into().unwrap()
    }
    pub fn fully_intersects(lhs: &Self, rhs: &Self) -> bool {
        lhs.0 <= rhs.0 && lhs.1 >= rhs.1 || rhs.0 <= lhs.0 && rhs.1 >= lhs.1
    }
    pub fn intersects(lhs: &Self, rhs: &Self) -> bool {
        lhs.1 >= rhs.0 && lhs.1 <= rhs.1 || rhs.1 >= lhs.0 && rhs.1 <= lhs.1
    }
}
impl<T> TryFrom<(T, T)> for Range
where
    T: TryInto<Id>,
{
    type Error = String;
    fn try_from((lower, upper): (T, T)) -> Result<Self, Self::Error> {
        if let (Ok(lower), Ok(upper)) = (lower.try_into(), upper.try_into()) {
            if lower <= upper {
                Ok(Range(lower, upper))
            } else {
                Err("Invalid range".into())
            }
        } else {
            Err("Invalid range".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str2ranges;

    use super::*;

    #[test]
    fn test_fully_intersects() {
        let ranges = str2ranges(crate::tests::INPUT);
        assert!(!Range::fully_intersects(&ranges[0].0, &ranges[0].1));
        assert!(!Range::fully_intersects(&ranges[1].0, &ranges[1].1));
        assert!(!Range::fully_intersects(&ranges[2].0, &ranges[2].1));
        assert!(Range::fully_intersects(&ranges[3].0, &ranges[3].1));
        assert!(Range::fully_intersects(&ranges[4].0, &ranges[4].1));
        assert!(!Range::fully_intersects(&ranges[5].0, &ranges[5].1));
    }

    #[test]
    fn test_intersects() {
        let ranges = str2ranges(crate::tests::INPUT);
        assert!(!Range::intersects(&ranges[0].0, &ranges[0].1));
        assert!(!Range::intersects(&ranges[1].0, &ranges[1].1));
        assert!(Range::intersects(&ranges[2].0, &ranges[2].1));
        assert!(Range::intersects(&ranges[3].0, &ranges[3].1));
        assert!(Range::intersects(&ranges[4].0, &ranges[4].1));
        assert!(Range::intersects(&ranges[5].0, &ranges[5].1));
    }
}
