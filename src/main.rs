use std::net::TcpListener;

use rusty_newsletter::startup::run;
use rusty_newsletter::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let lisener = TcpListener::bind(address).expect("Failed to bind to random port");
    let port = lisener.local_addr().unwrap().port();
    println!("127.0.0.1:{}",port);
    let server = run(lisener).expect("failed to start server");
    server.await
}
