use bluest::{pairing::NoInputOutputPairingAgent, Adapter, Uuid, Characteristic};
use futures_lite::StreamExt;
use instax_pal::*;
use std::error::Error;
use num_traits::FromPrimitive;

// UART-like GATT service
// Commands are sent to INSTAX_WRITE_UUID characteristic
// Responses are read from INSTAX_NOTIFY_UUID characteristic
// Reference: https://github.com/jpwsutton/instax_api/issues/21#issuecomment-770462168
const INSTAX_SERVICE_UUID: Uuid = Uuid::from_u128(0x70954782_2d83_473d_9e5f_81e1d02d5273);
const INSTAX_WRITE_UUID: Uuid = Uuid::from_u128(0x70954783_2d83_473d_9e5f_81e1d02d5273);
const INSTAX_NOTIFY_UUID: Uuid = Uuid::from_u128(0x70954784_2d83_473d_9e5f_81e1d02d5273);

#[derive(Debug)]
struct Packet {
    direction: Direction,
    size: u16,
    sid: SID,
    msg_type: u8,
    data: Vec<u8>
}
impl Packet {
    fn pack(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();
        packet.extend((self.direction as u16).to_be_bytes());
        packet.extend(self.size.to_be_bytes());
        packet.extend((self.sid as u16).to_be_bytes());
        packet.push(self.msg_type);
        packet.push(255 - packet.iter().sum::<u8>() as u8);
        packet
    }
    fn unpack(msg: &Vec<u8>) -> Self {
        Packet {
            direction: FromPrimitive::from_u16(u16::from_be_bytes(msg[0..2].try_into().unwrap())).unwrap(),
            size: u16::from_be_bytes(msg[2..4].try_into().unwrap()),
            sid: FromPrimitive::from_u16(u16::from_be_bytes(msg[4..6].try_into().unwrap())).unwrap(),
            msg_type: msg[7],
            data: msg[8..].to_vec()
        }
    }
    fn with_type(sid: SID, msg_type: u8) -> Self {
        Packet {
            direction: Direction::TO,
            size: 8, // Direction(2) + Size (2) + SID (2) + Type (1) + Checksum (1)
            sid,
            msg_type,
            data: vec![],
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")?;
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
    let write_char = characteristics
        .iter()
        .find(|x| x.uuid() == INSTAX_WRITE_UUID)
        .ok_or("write characteristic not found")?;
    let notify_char = characteristics
        .iter()
        .find(|x| x.uuid() == INSTAX_NOTIFY_UUID)
        .ok_or("notify characteristic not found")?;
    get_battery_level(write_char, notify_char).await;
    Ok(())
}

async fn get_battery_level(write_char: &Characteristic, notify_char: &Characteristic) {
    let packet = Packet::with_type(
        SID::SUPPORT_FUNCTION_INFO,
        SupportFunctionInfoType::BATTERY_INFO as u8
    );
    write_char.write(&packet.pack()).await.unwrap();
    let mut updates = notify_char.notify().await.unwrap();
    while let Some(msg) = updates.next().await {
        let response = Packet::unpack(&msg.unwrap());
        println!("Battery level: {}%", response.data[1]);
        break
    }
}
