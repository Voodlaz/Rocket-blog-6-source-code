use crate::posts::forms::{NewPostForm};

use rocket_contrib::templates::Template;
use std::collections::HashMap;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

#[get("/new_post")]
pub fn new_post<'signal>(signal: Option<FlashMessage<'_,'_>>) -> Template {
    dbg!(&signal);
    let mut context: HashMap<&str, String> = HashMap::new();
    if let Some(signal) = signal {
        let signal_error = signal.msg();
        context.insert("validation_error".into(), signal_error.into());
    }
    Template::render("new_post", context)
}

#[post("/new_post", data="<form>")]
pub fn new_post_form(form: Form<NewPostForm>) -> Flash<Redirect> {
    let form = form.into_inner();
    if form.name != "" && form.body != "" {
        Flash::success(Redirect::to("/new_post"), "all working")
    } else {
        Flash::error(Redirect::to("/new_post"), "Server error 500")
    }
}
