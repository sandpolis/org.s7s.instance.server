use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SessionToken {

}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {

	/// The user's unchangable username
	pub username: String,

	/// The user's password hash
	pub password: String,

	/// The user's TOTP secret token
	pub totp_token: String,

	/// The user's optional email address
	pub email: String,

	/// The user's optional phone number
	pub phone: String,

	#[serde(skip_serializing)]
	pub sessions: Vec<SessionToken>,
}