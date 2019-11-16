use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct User {
    pub uid: u32,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, !> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| build_user(id))
            .or_forward(())
    }
}

pub fn build_user(uid: u32) -> User {
    User {
        uid: uid,
    }
}

pub fn user_from_login(user: &String, password: &String) -> Result<User, &'static str> {
    match user == "matt" && password == "password" {
        true => Ok(build_user(1)),
        false => Err("Invalid username/password")
    }
}