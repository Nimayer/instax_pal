use std::error::Error;
use std::time::Duration;
use bluest::{Adapter, Uuid, pairing::NoInputOutputPairingAgent};
use futures_lite::{StreamExt};
use instax_pal::*;

// UART-like GATT service
// Commands are sent to INSTAX_WRITE_UUID characteristic
// Responses are read from INSTAX_NOTIFY_UUID characteristic
// Reference: https://github.com/jpwsutton/instax_api/issues/21#issuecomment-770462168
const INSTAX_SERVICE_UUID: Uuid = Uuid::from_u128(0x70954782_2d83_473d_9e5f_81e1d02d5273);
const INSTAX_WRITE_UUID: Uuid = Uuid::from_u128(0x70954783_2d83_473d_9e5f_81e1d02d5273);
const INSTAX_NOTIFY_UUID: Uuid = Uuid::from_u128(0x70954784_2d83_473d_9e5f_81e1d02d5273);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;
    println!("Searching Instax device");
    let device = adapter
        .discover_devices(&[INSTAX_SERVICE_UUID])
        .await?
        .next()
        .await
        .ok_or("Failed to discover device")??;
    println!(
        "found device: {} ({:?})",
        device.name().as_deref().unwrap_or("(unknown)"),
        device.id()
    );
    // HACK: on some laptops we need to repeat pairing, otherwise we can't connect
    if device.is_paired().await? {
        println!("Repeating device pairing");
        device.unpair().await?
    }
    device.pair_with_agent(&NoInputOutputPairingAgent).await?;
    adapter.connect_device(&device).await?;
    println!("connected!");
    let service = match device
        .discover_services_with_uuid(INSTAX_SERVICE_UUID)
        .await?
        .get(0)
    {
        Some(service) => service.clone(),
        None => return Err("Service not found".into()),
    };
    let characteristics = service.characteristics().await?;
    let write_characteristic = characteristics
        .iter()
        .find(|x| x.uuid() == INSTAX_WRITE_UUID)
        .ok_or("write characteristic not found")?;
    let notify_characteristic = characteristics
        .iter()
        .find(|x| x.uuid() == INSTAX_NOTIFY_UUID)
        .ok_or("notify characteristic not found")?;
    let packet = prepare_packet(instax_pal::SUPPORT_FUNCTION_INFO, instax_pal::SupportFunctionInfoType::BATTERY_INFO as u8);
    dbg!(&packet);
    write_characteristic.write(&packet).await?;
    let mut updates = notify_characteristic.notify().await?;
    while let Some(val) = updates.next().await {
        println!("Message: {:?}", val?);
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
