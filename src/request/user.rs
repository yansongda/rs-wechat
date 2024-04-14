use crate::model::result::Error;
use crate::model::user::{UpdateUser, User};
use crate::request::Validator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub code: Option<String>,
}

impl Validator for LoginRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.code.is_none() {
            return Err(Error::Params(Some("小程序错误：登录秘钥不能为空")));
        }

        if self.code.to_owned().unwrap().chars().count() < 8 {
            return Err(Error::Params(Some("小程序错误：登录秘钥必须大于 8 位")));
        }

        Ok(self.code.to_owned().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub open_id: String,
}

impl From<User> for LoginResponse {
    fn from(user: User) -> Self {
        Self {
            open_id: user.open_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRequest {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl Validator for UpdateRequest {
    type Data = UpdateUser;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.avatar.is_some() && self.avatar.to_owned().unwrap().len() < 8 {
            return Err(Error::Params(Some("头像不符合规范")));
        }

        if self.nickname.is_some() {
            let nickname = self.nickname.to_owned().unwrap();

            if nickname.is_empty() || nickname.chars().count() > 10 {
                return Err(Error::Params(Some("昵称长度应为 1~10 之间，请正确填写")));
            }
        }

        if self.slogan.is_some() {
            let slogan = self.slogan.to_owned().unwrap();

            if slogan.is_empty() || slogan.chars().count() > 50 {
                return Err(Error::Params(Some("slogan 长度应为 1~50 之间，请正确填写")));
            }
        }

        Ok(Self::Data::from(self.to_owned()))
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl From<User> for DetailResponse {
    fn from(user: User) -> Self {
        Self {
            avatar: user.avatar,
            nickname: user.nickname,
            slogan: user.slogan,
        }
    }
}
