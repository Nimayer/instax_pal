use bluez_async::{BluetoothEvent, BluetoothSession, CharacteristicEvent};
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;
use instax_pal::*;

// UART-like GATT service
// Commands are sent to WRITE_UUID characteristic
// Responses are read from NOTIFY_UUID characteristic
// Reference: https://github.com/jpwsutton/instax_api/issues/21#issuecomment-770462168
const SERVICE_UUID: Uuid = Uuid::from_u128(0x70954782_2d83_473d_9e5f_81e1d02d5273);
const WRITE_UUID: Uuid = Uuid::from_u128(0x70954783_2d83_473d_9e5f_81e1d02d5273);
const NOTIFY_UUID: Uuid = Uuid::from_u128(0x70954784_2d83_473d_9e5f_81e1d02d5273);

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    pretty_env_logger::init();
    // Create a new session. This establishes the D-Bus connection to talk to BlueZ. In this case we
    // ignore the join handle, as we don't intend to run indefinitely.
    let (_, session) = BluetoothSession::new().await?;
    // Start scanning for Bluetooth devices, and wait a few seconds for some to be discovered.
    session.start_discovery().await?;
    time::sleep(Duration::from_secs(5)).await;
    session.stop_discovery().await?;
    let devices = session.get_devices().await?;
    let device = devices
        .into_iter()
        .filter(|device| device.name.is_some())
        .find(|device| device.name.as_ref().unwrap().starts_with("INSTAX-") && device.name.as_ref().unwrap().ends_with("(IOS)"))
        .unwrap();
    println!("Instax camera found: {}", &device.name.unwrap());
    session.connect(&device.id).await?;
    println!("Connected!");
    let service = session
        .get_service_by_uuid(
            &device.id,
            SERVICE_UUID,
        )
        .await?;
    let notify_characteristic = session.get_characteristic_by_uuid(&service.id, NOTIFY_UUID).await?;
    // Subscribe to notifications on the characteristic and print them out.
    let mut events = session.characteristic_event_stream(&notify_characteristic.id).await?;
    session.start_notify(&notify_characteristic.id).await?;
    let write_characteristic = session.get_characteristic_by_uuid(&service.id, WRITE_UUID).await?;
    let packet = prepare_packet(instax_pal::SUPPORT_FUNCTION_INFO, instax_pal::SupportFunctionInfoType::BATTERY_INFO as u8);
    dbg!(&packet);
    let response = session.write_characteristic_value(&write_characteristic.id, packet).await?;
    println!("Packet sent!");
    dbg!(response);
    println!("Waiting for notifications");
    while let Some(event) = events.next().await {
        if let BluetoothEvent::Characteristic {
            id,
            event: CharacteristicEvent::Value { value },
        } = event
        {
            println!("Update from {}: {:?}", id, value);
        } else {
            println!("Other event {:?}", event)
        }
    }
    Ok(())
}

fn prepare_packet(sid: u16, msg_type: u8) -> Vec<u8> {
    let mut packet: Vec<u8> = Vec::new();
    packet.extend(DIRECTION_TO.to_be_bytes());
    let size: u16 = 8; // Direction(2) + Size (2) + SID (2) + Type (1) + Checksum (1)
    packet.extend(size.to_be_bytes());
    packet.extend(sid.to_be_bytes());
    packet.push(msg_type);
    let checksum: u8 = 255 - packet.iter().sum::<u8>() as u8;
    packet.push(checksum);
    packet
}
