use std::default::Default;
use std::sync::Arc;

use bluer::Uuid;
use tokio::{
    main, spawn,
    sync::{mpsc, Mutex},
    task,
};

use van_colleague::{
    ble_server::{slider_read_write_service, voice_command_service,},
    session::Session,
};

const BEDROOM_UUID: Uuid = Uuid::from_u128(0x0584507902e74f44b67902b90775abda);
const KITCHEN_UUID: Uuid = Uuid::from_u128(0x36bc0fe1b00742809ec6b36c8bc98537);
const VOICE_UUID: Uuid = Uuid::from_u128(0x7e1be1ebf9844e17b0f1049e02a39567);

#[tokio::main]
async fn main() {
    let mut session = Session {
        ..Default::default()
    };

    let (cli_command, located_devices) = session.setup().await;

    let ble_services = vec![
        slider_read_write_service(BEDROOM_UUID, Arc::clone(&session.shared_ble_command)),
        slider_read_write_service(KITCHEN_UUID, Arc::clone(&session.shared_ble_command)),
        voice_command_service(VOICE_UUID, Arc::clone(&session.shared_ble_command), located_devices.clone()),
    ];    

    session.run(KITCHEN_UUID, cli_command, located_devices, ble_services).await;
}
