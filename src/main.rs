#![feature(proc_macro_hygiene, decl_macro, never_type)]

use std::collections::HashMap;

#[macro_use] extern crate chrono;

#[macro_use] extern crate rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies};

use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod user;
use user::{User, user_from_login};

#[derive(FromForm)]
struct Login {
    username: String,
    password: String
}

#[post("/login", data = "<login>")]
fn login(mut cookies: Cookies, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    match user_from_login(&login.username, &login.password) {
        Ok(u) => {
            cookies.remove_private(Cookie::named("user_id"));
            cookies.add_private(Cookie::new("user_id", u.uid.to_string()));
            Ok(Redirect::to(uri!(index)))
        }
        Err(e) => Err(Flash::error(Redirect::to(uri!(login_page)), e))

    }
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[get("/login")]
fn login_user(_user: User) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)
}

#[get("/")]
fn user_index(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("user_id", user.uid);
    Template::render("index", &context)
}

#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to(uri!(login_page))
}

#[get("/brew")]
fn user_brew(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("user_id", user.uid);
    Template::render("brew", &context)
}
#[get("/brew", rank = 2)]
fn brew() -> Redirect {
    Redirect::to(uri!(login_page))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(SpaceHelmet::default())
        .mount("/static", StaticFiles::from("./static"))
        .mount("/", routes![index, user_index, login, logout, login_user, login_page, user_brew, brew])
}

fn main() {
    rocket().launch();
}

