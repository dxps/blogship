use rocket::routes;

use crate::errors::AppError;
use crate::routes::common::get_index;

pub async fn new_server() -> Result<(), AppError> {
    let srv = rocket::build().mount("/", routes![get_index]);
    match srv.launch().await {
        Ok(_) => {
            println!("Started");
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
