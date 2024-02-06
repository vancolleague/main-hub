use std::default::Default;
use std::sync::Arc;

use bluer::Uuid;
use tokio::{
    main, spawn,
    sync::{mpsc, Mutex},
    task,
};

use device::Behavior;
use van_colleague::{
    ble_server::{slider_service, voice_service},
    session::{get_user_args, Session},
};

const VOICE_UUID: Uuid = Uuid::from_u128(0x7e1be1ebf9844e17b0f1049e02a39567);

#[tokio::main]
async fn main() {
    let mut session = Session {
        ..Default::default()
    };

    let (cli_command, located_devices) = session.setup().await;

    let mut ble_services = Vec::new();
    match cli_command.subcommand() {
        Some(("run", _)) => {
            for (uuid, ld) in located_devices.iter() {
                match ld.device.behavior {
                    Behavior::Slider => {
                        ble_services.push(slider_service(uuid.clone(), Arc::clone(&session.shared_ble_command)));
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    ble_services.push(voice_service(
        VOICE_UUID.clone(),
        Arc::clone(&session.shared_ble_command),
        located_devices.clone(),
    ));

    session
        .run(VOICE_UUID, cli_command, located_devices, ble_services)
        .await;
}
