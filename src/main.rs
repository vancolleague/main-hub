use van_colleague::session::Session;

#[tokio::main]
async fn main() {
    let mut session = Session {
        ble_name: "VanColleague".to_string(),
        ..Default::default()
    };

    session.run().await;
}
