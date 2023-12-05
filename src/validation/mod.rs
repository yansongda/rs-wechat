use crate::model::result::Result;

pub trait Validator: Sized {
    type Data: for<'a> From<&'a Self>;

    fn validate(&self) -> Result<Self::Data>;
}
