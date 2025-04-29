use binrw::binread;

// ZiPatch have practically no header, it's chunks all the way down - this is
// for consistency more than anything else.
#[binread]
#[br(big, magic = b"\x91ZIPATCH\x0D\x0A\x1A\x0A")]
#[derive(Debug)]
pub struct Header {}
