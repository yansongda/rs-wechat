use crate::model::result::Error;
use crate::model::totp::UpdateTotp;
use crate::request::Validator;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DetailRequest {
    pub id: Option<i64>,
}

impl Validator for DetailRequest {
    type Data = i64;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::Params(Some("id 不能为空")));
        }

        Ok(self.id.unwrap())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequest {
    pub uri: Option<String>,
}

impl Validator for CreateRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.uri.is_none() {
            return Err(Error::Params(Some("totp 链接不能为空")));
        }

        if !self.uri.clone().unwrap().starts_with("otpauth://totp/") {
            return Err(Error::Params(Some("totp 链接格式错误")));
        }

        Ok(self.uri.clone().unwrap())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub issuer: Option<String>,
    pub username: Option<String>,
}

impl Validator for UpdateRequest {
    type Data = UpdateTotp;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::Params(Some("id 不能为空")));
        }

        if self.username.is_none() || self.username.clone().unwrap().is_empty() {
            return Err(Error::Params(Some("username 不能为空")));
        }

        Ok(Self::Data::from(self))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRequest {
    pub id: Option<i64>,
}

impl Validator for DeleteRequest {
    type Data = i64;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::Params(Some("id 不能为空")));
        }

        Ok(self.id.unwrap())
    }
}
