use std::collections::HashMap;

use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use rocket_contrib::templates::Template;

#[macro_export]
macro_rules! message_uri {
    ($($t:tt)*) => (rocket::uri!("/message", $crate::message:: $($t)*))
}

pub use message_uri as uri;

#[post("/", data = "<message>")]
fn submit(cookies: &CookieJar<'_>, message: Form<&str>) -> Redirect {
    cookies.add(Cookie::new("message", message.to_string()));
    Redirect::to(uri!(index))
}

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Template {
    let cookie = cookies.get("message");
    let mut context = HashMap::new();
    if let Some(ref cookie) = cookie {
        context.insert("message", cookie.value());
    }

    Template::render("message", &context)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![submit, index]
}
