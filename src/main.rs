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
enum PacketType {
    Sid = 0,
    Type = 1,
    Data = 3,
}

#[derive(Debug)]
struct Packet {
    p_type: PacketType,
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
        if matches!(self.p_type, PacketType::Type) {
            packet.push(self.msg_type);
        }
        if matches!(self.p_type, PacketType::Data) {
            packet.extend(&self.data);
        }
        // Add checksum
        let checksum: u8 = 255 - packet.iter().fold(0, |a: u8, &b| a.wrapping_add(b));
        packet.push(checksum);
        packet
    }
    fn unpack(msg: &Vec<u8>) -> Self {
        Packet {
            p_type : match msg.len() {
                    0..=7 => panic!("ERROR: Packet too short. len:{}", msg.len()),
                    8 => PacketType::Sid,
                    9 => PacketType::Type,
                    _ => PacketType::Data,
                },
            direction: FromPrimitive::from_u16(u16::from_be_bytes(msg[0..2].try_into().unwrap())).unwrap(),
            size: u16::from_be_bytes(msg[2..4].try_into().unwrap()),
            sid: FromPrimitive::from_u16(u16::from_be_bytes(msg[4..6].try_into().unwrap())).unwrap(),
            msg_type: msg[7],
            data: msg[8..].to_vec()
        }
    }
    fn with_sid(sid: SID) -> Self {
        Packet {
            p_type: PacketType::Sid,
            direction: Direction::TO,
            size: 7, // Direction(2) + Size (2) + SID (2) + Checksum (1)
            sid,
            msg_type: 0,
            data: vec![],
        }
    }
    fn with_type(sid: SID, msg_type: u8) -> Self {
        Packet {
            p_type: PacketType::Type,
            direction: Direction::TO,
            size: 8, // Direction(2) + Size (2) + SID (2) + Type (1) + Checksum (1)
            sid,
            msg_type,
            data: vec![],
        }
    }
    fn with_data(sid: SID, data: Vec<u8>) -> Self {
        Packet {
            p_type: PacketType::Data,
            direction: Direction::TO,
            size: 7 + data.len() as u16, // Direction(2) + Size (2) + SID (2) + Payload (N) + Checksum (1)
            sid,
            msg_type: 0,
            data: data,
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
    get_battery_info(write_char, notify_char).await;
    // get_function_info(write_char, notify_char).await;
    automatic_photo_upload(write_char, notify_char).await;
    Ok(())
}

async fn send_packet(write_char: &Characteristic, packet: Packet) {
    let data = packet.pack();
    println!("SENT: {:x?}", &data);
    write_char.write(&data).await.unwrap();
}

async fn receive_packet(notify_char: &Characteristic) -> Option<Packet> {
    let mut updates = notify_char.notify().await.unwrap();
    while let Some(msg) = updates.next().await {
        let data = &msg.unwrap();
        println!("RECV: {:x?}", &data);
        let packet = Packet::unpack(&data);
        return Some(packet);
    }
    None
}

async fn get_battery_info(write_char: &Characteristic, notify_char: &Characteristic) {
    let packet = Packet::with_type(
        SID::SUPPORT_FUNCTION_INFO,
        SupportFunctionInfoType::BATTERY_INFO as u8
    );
    send_packet(write_char, packet).await;
    let response = receive_packet(notify_char).await.unwrap();
    println!("Battery level: {}%", response.data[1]);
}

async fn get_function_info(write_char: &Characteristic, notify_char: &Characteristic) {
    let packet = Packet::with_type(
        SID::SUPPORT_FUNCTION_INFO,
        SupportFunctionInfoType::CAMERA_FUNCTION_INFO as u8
    );
    send_packet(write_char, packet).await;
    let response = receive_packet(notify_char).await.unwrap();
}

async fn automatic_photo_upload(write_char: &Characteristic, notify_char: &Characteristic) {
    println!("Auto upload info");
    let packet = Packet::with_sid(SID::IMAGE_AUTO_UPLOAD_INFO);
    send_packet(write_char, packet).await;
    let response = receive_packet(notify_char).await.unwrap();
    println!("Auto upload start");
    let packet = Packet::with_data(SID::IMAGE_AUTO_UPLOAD_START, vec![0;4]);
    send_packet(write_char, packet).await;
    let response = receive_packet(notify_char).await.unwrap();
    println!("Auto upload data");
    let num_frames = response.data[1];
    for frame in 0..num_frames {
        let frame_num = (frame as u32).to_be_bytes().to_vec();
        let packet = Packet::with_data(SID::IMAGE_AUTO_UPLOAD_DATA, frame_num);
        send_packet(write_char, packet).await;
        let response = receive_packet(notify_char).await.unwrap();
        let response = receive_packet(notify_char).await.unwrap();
        let response = receive_packet(notify_char).await.unwrap();
    }
}
