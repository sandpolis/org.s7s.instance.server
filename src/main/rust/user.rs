use crate::DbConnection;
use anyhow::Result;
use ring::pbkdf2;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize)]
pub struct SessionToken {}

#[derive(Clone, Serialize, Deserialize)]
pub struct PasswordHash {
    pub iterations: NonZeroU32,

    pub salt: String,

    pub hash: String,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct User {
    /// The user's unchangable username
    #[validate(length(min = 4), length(max = 20))]
    pub username: String,

    /// Whether the user is an admin
    pub admin: bool,

    /// The user's password hash
    pub password: PasswordHash,

    /// The user's TOTP secret token
    pub totp_secret: Option<String>,

    /// The user's optional email address
    #[validate(email)]
    pub email: Option<String>,

    /// The user's optional phone number
    #[validate(phone)]
    pub phone: Option<String>,

    pub expiration: Option<i64>,

    #[serde(skip)]
    pub sessions: Vec<SessionToken>,
}

impl User {
    pub fn verify_login(&self, password: &str, totp_token: i32) -> bool {
        if !pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA256,
            self.password.iterations,
            self.password.salt.as_bytes(),
            password.as_bytes(),
            self.password.hash.as_bytes(),
        )
        .is_ok()
        {
            return false;
        }

        return true;
    }
}

pub struct UserBuilder(User);

impl UserBuilder {
    pub fn new(username: &str) -> Self {
        UserBuilder {
            0: User {
                username: username.to_string(),
                password: PasswordHash {
                    iterations: NonZeroU32::new(1).unwrap(),
                    salt: String::new(),
                    hash: String::new(),
                },
                totp_token: None,
                email: None,
                phone: None,
                expiration: None,
                sessions: vec![],
            },
        }
    }

    pub fn build(&self) -> Result<User> {
        self.0.validate()?;
        Ok(self.0)
    }
}

impl From<core_protocol::core::protocol::get_user_response::User> for User {
    fn from(user: core_protocol::core::protocol::get_user_response::User) -> Self {}
}
