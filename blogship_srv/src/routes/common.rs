use rocket::get;

#[get("/")]
pub async fn get_index() -> &'static str {
    "Welcome!"
}
