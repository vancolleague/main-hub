use van_colleague::session::Session;

#[tokio::main]
async fn main() {
    let mut session = Session {
        ..Default::default()
    };

    session.run().await;
}
