pub mod totp;
pub mod user;
pub mod shortlink;

use crate::model::result::Result;

pub trait Validator {
    type Data;

    fn validate(&self) -> Result<Self::Data>;
}
