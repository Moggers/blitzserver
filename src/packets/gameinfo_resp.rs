use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
#[derive(Clone, Debug)]
pub struct GameInfoResp {
    pub unk1: u32,
    pub game_state: u8,
    pub game_name: String,
    pub era: i32,
    pub unk2: u32,
    pub disciples: bool,
    pub unk3: u8,
    pub milliseconds_to_host: Option<u32>,
    pub unk4: u16,
    pub nation_statuses: std::collections::HashMap<i32, u8>,
    pub turn_statuses: std::collections::HashMap<i32, u8>,
    pub remaining: Vec<u8>,
    pub turn_number: u32,
    pub turnkey: u32,
}

const DISCIPLES_BIT_ID: u32 = 0b10000000;

impl GameInfoResp {
    pub fn from_reader<R: std::io::Read>(r: &mut R) -> GameInfoResp {
        let unk1 = r.read_u32::<LittleEndian>().unwrap();
        let game_state = r.read_u8().unwrap();
        let mut buf = [0u8; 1];
        let mut name = String::new();
        loop {
            r.read_exact(&mut buf).unwrap();
            if buf[0] == 0 {
                break;
            }
            name.push(buf[0] as char);
        }
        let era = r.read_u8().unwrap();
        let unk2 = r.read_u32::<LittleEndian>().unwrap();
        let disciples = unk2 & DISCIPLES_BIT_ID == DISCIPLES_BIT_ID;
        let unk3 = r.read_u8().unwrap();
        let milliseconds_to_host = match r.read_u32::<LittleEndian>() {
            Ok(0xffffffff) => None,
            Ok(a) => Some(a),
            _ => None,
        };
        let unk4 = r.read_u16::<LittleEndian>().unwrap();
        let mut nation_statuses: std::collections::HashMap<i32, u8> =
            std::collections::HashMap::new();
        for i in 1..=250 {
            match r.read_u8() {
                Ok(1) => {
                    nation_statuses.insert(i, 1);
                }
                _ => {}
            }
        }
        let mut turn_statuses: std::collections::HashMap<i32, u8> =
            std::collections::HashMap::new();
        for i in 1..=250 {
            match r.read_u8() {
                Ok(1) => {
                    turn_statuses.insert(i, 1);
                }
                _ => {}
            }
        }
        let mut remaining = vec![];
        r.read_to_end(&mut remaining).unwrap();
        GameInfoResp {
            unk1,
            game_state,
            game_name: name,
            era: era.into(),
            unk2,
            remaining,
            disciples,
            unk3,
            milliseconds_to_host,
            unk4,
            nation_statuses,
            turn_statuses,
            turn_number: 0,
            turnkey: 0,
        }
    }
}
impl crate::packets::BodyContents for GameInfoResp {
    const ID: u8 = 0x4;

    fn write<W: std::io::Write>(&self, w: &mut W) {
        w.write(&[0xc3, 0x1, 0, 0]).unwrap();
        w.write_u8(self.game_state).unwrap();
        w.write(self.game_name.as_bytes()).unwrap();
        w.write(&[0u8]).unwrap(); // Null terminator
        w.write(&[self.era as u8]).unwrap();
        let mut unk_bit_map = 0x840;
        if self.disciples {
            unk_bit_map = unk_bit_map | DISCIPLES_BIT_ID;
        }
        w.write_u32::<LittleEndian>(unk_bit_map).unwrap();
        w.write_u8(0x2d).unwrap();
        match self.milliseconds_to_host {
            Some(t) => w.write_u32::<LittleEndian>(t).unwrap(),
            None => w.write_u32::<LittleEndian>(0xffffffff).unwrap(),
        };

        w.write_all(&[0, 0]).unwrap();
        for i in 1..=250 {
            match self.nation_statuses.get(&i) {
                Some(1) => w.write_u8(1).unwrap(),
                _ => w.write_u8(0).unwrap(),
            }
        }
        for i in 1..250 {
            match self.turn_statuses.get(&i) {
                Some(t) => w.write_u8(*t).unwrap(),
                _ => w.write_u8(0).unwrap(),
            }
        }
        w.write_all(&[0; 250]).unwrap();
        match self.turn_number {
            0 => {
                w.write_all(&[0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00])
                    .unwrap();
            }
            t => {
                w.write_u32::<LittleEndian>(t).unwrap();
                w.write_u8(0).unwrap();
                w.write_u32::<LittleEndian>(self.turnkey).unwrap();
            }
        }
    }
}
