use crate::model::result::Error;

pub fn find(_open_id: String) -> Result<(), Error> {
    todo!()
}

pub fn find_one(_id: i32) -> Result<(), Error> {
    todo!()
}

pub fn create(
    _user_id: i32,
    _username: String,
    _issuer: String,
    _secret: String,
) -> Result<(), Error> {
    todo!()
}

pub fn update(_id: i32, _username: String, _issuer: String) -> Result<(), Error> {
    todo!()
}
