use blogship_srv::server::new_server;

#[rocket::main]
async fn main() {
    // Server is instantiated and started in one step.
    let server = new_server().await;
    if server.is_err() {
        println!("Server error: {}", server.err().unwrap())
    }
}
