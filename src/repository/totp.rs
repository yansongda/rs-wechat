use crate::model::result::Error;

pub fn find(open_id: String) -> Result<(), Error> {
    todo!()
}

pub fn find_one(id: i32) -> Result<(), Error> {
    todo!()
}

pub fn create(user_id: i32, username: String, issuer: String, secret: String) -> Result<(), Error> {
    todo!()
}

pub fn update(id: i32, username: String, issuer: String) -> Result<(), Error> {
    todo!()
}
