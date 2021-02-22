
#[derive(Debug, Clone)]
pub struct PAReq {
}

impl PAReq {
    pub fn from_reader<R: std::io::Read>(_r: &mut R) -> PAReq {
        PAReq {}
    }
}

impl crate::packets::BodyContents for PAReq {
    const ID: u8 = 0x1;
    fn write<W: std::io::Write>(&self, _w: &mut W) {}
}
