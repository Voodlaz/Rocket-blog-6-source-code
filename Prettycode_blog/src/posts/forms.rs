use rocket::http::RawStr;

#[derive(FromForm)]
pub struct NewPostForm<'f> {
    pub name: String,
    pub body: &'f RawStr
}
