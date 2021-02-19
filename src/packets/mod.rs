use crate::num_derive::{FromPrimitive, ToPrimitive};
use crate::num_traits::{FromPrimitive, ToPrimitive};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::Write;

pub mod astralpacket_req;
pub mod astralpacket_resp;
pub mod gameinfo_req;
pub mod gameinfo_resp;
pub mod heartbeat_req;
pub mod uploadpretender_req;
pub mod disconnect_req;
pub mod disconnect_resp;
pub mod startgame_req;
pub mod pa_req;
pub mod pa_resp;
pub mod unknown_req;
pub mod passwords_req;
pub mod twohcrc_req;
pub mod passwords_resp;
pub mod twohcrc_resp;
pub mod loadingmessage_resp;
pub mod trn_req;
pub mod trn_resp;
pub mod map_req;
pub mod map_resp;
pub mod mapfile_req;
pub mod mapfile_resp;

pub use astralpacket_req::AstralPacketReq;
pub use astralpacket_resp::AstralPacketResp;
pub use gameinfo_req::GameInfoReq;
pub use gameinfo_resp::GameInfoResp;
pub use heartbeat_req::HeartbeatReq;
pub use uploadpretender_req::UploadPretenderReq;
pub use disconnect_req::DisconnectReq;
pub use disconnect_resp::DisconnectResp;
pub use startgame_req::StartGameReq;
pub use pa_req::PAReq;
pub use pa_resp::PAResp;
pub use unknown_req::UnknownReq;
pub use passwords_req::PasswordsReq;
pub use twohcrc_req::TwoHCrcReq;
pub use passwords_resp::PasswordsResp;
pub use twohcrc_resp::TwoHCrcResp;
pub use loadingmessage_resp::LoadingMessageResp;
pub use trn_req::TrnReq;
pub use trn_resp::TrnResp;
pub use map_req::MapReq;
pub use map_resp::MapResp;
pub use mapfile_req::MapFileReq;
pub use mapfile_resp::MapFileResp;

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_connection_req() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/client_req")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }

    #[test]
    fn deserialize_connection_resp_1() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ea_aaa")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }

    #[test]
    fn deserialize_connection_resp_2() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ma_bbb")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_3() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ma_aaa_wh")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_4() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ma_aaa_noclientstart")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_5() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ma_aaa_clientstart")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_6() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ma_aaa_ulm")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_7() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_la_aaa_erytheia")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_8() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ea_disciples")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_9() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ea_easy_research")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_10() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ea_hel_closed")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_11() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/server_resp_ea_started")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_12() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/their_garbage_resp")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_13() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/similar_garbage_resp")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_14() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/timer")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_connection_resp_15() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/connect_lobby/timer_1")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
    #[test]
    fn deserialize_game_start_1() {
        let test_packet = crate::packets::Packet::from_reader(&mut std::io::BufReader::new(
            &include_bytes!("../../pktdmps/game_start/real_server")[..],
        ));
        println!("Packet: {:x?}", test_packet);
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub header: Header,
    pub body: Body,
}

impl Packet {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> Packet {
        let header = Header::from_reader(r);
        let mut body_buf: Vec<u8> = vec![0u8; header.length as usize];
        r.read_exact(&mut body_buf).unwrap();
        let mut reader = &body_buf[..];
        let body = if header.compression == CompressionType::Zlib {
            let len = reader.read_u32::<LittleEndian>().unwrap();
            Body::from_reader(&mut ZlibDecoder::new(reader))
        } else {
            Body::from_reader(&mut reader)
        };
        Packet { header, body }
    }
}

#[derive(Debug, Clone, PartialEq, ToPrimitive, FromPrimitive)]
enum CompressionType {
    Na = 0x48,
    Zlib = 0x4a,
}

#[derive(Debug, Clone)]
pub struct Header {
    unk: u8,
    compression: CompressionType,
    pub length: u32,
}

impl Header {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> Header {
        let unk: u8 = r.read_u8().unwrap();
        let compression = FromPrimitive::from_u8(r.read_u8().unwrap()).unwrap();
        let length: u32 = r.read_u32::<LittleEndian>().unwrap();
        Header {
            unk,
            compression,
            length,
        }
    }
    pub fn write<W: std::io::Write>(&self, w: &mut W) {
        w.write_u8(0x66).unwrap();
        w.write_u8(CompressionType::Zlib.to_u8().unwrap()).unwrap();
        w.write_u32::<LittleEndian>(self.length).unwrap();
    }
}

#[derive(Debug, Clone)]
pub enum Body {
    HeartbeatReq(HeartbeatReq),
    UploadPretenderReq(UploadPretenderReq),
    AstralPacketReq(AstralPacketReq),
    AstralPacketResp(AstralPacketResp),
    GameInfoReq(GameInfoReq),
    GameInfoResp(GameInfoResp),
    DisconnectReq(DisconnectReq),
    StartGameReq(StartGameReq),
    PAReq(PAReq),
    PAResp(PAResp),
    LoadingMessageResp(LoadingMessageResp),
    UnknownReq(UnknownReq),
    PasswordsReq(PasswordsReq),
    TwoHCrcReq(TwoHCrcReq),
    PasswordsResp(PasswordsResp),
    TwoHCrcResp(TwoHCrcResp),
    TrnReq(TrnReq),
    TrnResp(TrnResp),
    MapReq(MapReq),
    MapResp(MapResp),
    MapFileReq(MapFileReq),
    MapFileResp(MapFileResp)
}

impl Body {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> Body {
        match r.read_u8().unwrap() {
            UploadPretenderReq::ID => Body::UploadPretenderReq(UploadPretenderReq::from_reader(r)),
            HeartbeatReq::ID => Body::HeartbeatReq(HeartbeatReq::from_reader(r)),
            GameInfoReq::ID => Body::GameInfoReq(GameInfoReq::from_reader(r)),
            GameInfoResp::ID => Body::GameInfoResp(GameInfoResp::from_reader(r)),
            AstralPacketResp::ID => Body::AstralPacketResp(AstralPacketResp::from_reader(r)),
            AstralPacketReq::ID => Body::AstralPacketReq(AstralPacketReq::from_reader(r)),
            StartGameReq::ID => Body::StartGameReq(StartGameReq::from_reader(r)),
            DisconnectReq::ID => Body::DisconnectReq(DisconnectReq::from_reader(r)),
            PAReq::ID => Body::PAReq(PAReq::from_reader(r)),
            PAResp::ID => Body::PAResp(PAResp::from_reader(r)),
            UnknownReq::ID => Body::UnknownReq(UnknownReq::from_reader(r)),
            PasswordsReq::ID => Body::PasswordsReq(PasswordsReq::from_reader(r)),
            TwoHCrcReq::ID => Body::TwoHCrcReq(TwoHCrcReq::from_reader(r)),
            PasswordsResp::ID => Body::PasswordsResp(PasswordsResp::from_reader(r)),
            TwoHCrcResp::ID => Body::TwoHCrcResp(TwoHCrcResp::from_reader(r)),
            TrnReq::ID => Body::TrnReq(TrnReq::from_reader(r)),
            TrnResp::ID => Body::TrnResp(TrnResp::from_reader(r)),
            MapReq::ID => Body::MapReq(MapReq::from_reader(r)),
            MapResp::ID => Body::MapResp(MapResp::from_reader(r)),
            MapFileReq::ID => Body::MapFileReq(MapFileReq::from_reader(r)),
            MapFileResp::ID => Body::MapFileResp(MapFileResp::from_reader(r)),
            d => {
                let mut v = vec![];
                r.read_to_end(&mut v).unwrap();
                panic!(
                "What the fuck is that? What the FUCK is that? Mystery id {:x?}, full contents:\n{:x?}",
                d, v);
            }
        }
    }
    pub fn write<W: std::io::Write>(&self, w: &mut W) {
        match self {
            Self::PAReq(p) => p.write(w),
            Self::PAResp(p) => p.write(w),
            Self::DisconnectReq(p) => p.write(w),
            Self::UploadPretenderReq(p) => p.write(w),
            Self::HeartbeatReq(p) => p.write(w),
            Self::AstralPacketReq(p) => p.write(w),
            Self::AstralPacketResp(p) => p.write(w),
            Self::GameInfoReq(p) => p.write(w),
            Self::GameInfoResp(p) => p.write(w),
            Self::StartGameReq(p) => p.write(w),
            Self::UnknownReq(p) => p.write(w),
            Self::PasswordsReq(p) => p.write(w),
            Self::TwoHCrcReq(p) => p.write(w),
            Self::PasswordsResp(p) => p.write(w),
            Self::TwoHCrcResp(p) => p.write(w),
            Self::LoadingMessageResp(p) => p.write(w),
            Self::TrnReq(p) => p.write(w),
            Self::TrnResp(p) => p.write(w),
            Self::MapReq(p) => p.write(w),
            Self::MapResp(p) => p.write(w),
            Self::MapFileReq(p) => p.write(w),
            Self::MapFileResp(p) => p.write(w),
        }
    }
}

pub trait BodyContents {
    const ID: u8;

    fn write<W: std::io::Write>(&self, w: &mut W);

    fn write_packet<W: std::io::Write>(&self, w: &mut W) {
        let mut full: Vec<u8> = vec![];

        // Create body
        let mut body: Vec<u8> = vec![];
        self.write(&mut body);
        let mut packet_body: Vec<u8> = vec![];
        packet_body
            .write_u32::<LittleEndian>((body.len() + 5) as u32) // TODO: This cant be right
            .unwrap();

        let mut zlib = ZlibEncoder::new(packet_body, flate2::Compression::default());
        zlib.write_u8(Self::ID).unwrap();
        zlib.write_all(&body).unwrap();
        let packet_body = zlib.finish().unwrap();

        // Write header then body
        (Header {
            length: packet_body.len() as u32,
            compression: CompressionType::Zlib,
            unk: 0,
        })
        .write(&mut full);
        full.write_all(&packet_body[..]).unwrap();
        w.write_all(&full).unwrap();
    }
}
