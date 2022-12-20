type IdT = u16;
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Id(IdT);

impl From<IdT> for Id {
    fn from(value: IdT) -> Self {
        Self(value)
    }
}
impl TryFrom<&str> for Id {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(v) = IdT::from_str_radix(value, 10) {
            Ok(v.into())
        } else {
            Err("Invalid Id".into())
        }
    }
}
