use testground::client::Client;
use utility_a::greeting;

#[tokio::main]
async fn main() {
    let client = Client::new_and_init().await.unwrap();
    client.record_message(greeting());
    client.record_success().await.unwrap();
}
