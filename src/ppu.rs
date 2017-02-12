#[derive(Clone)]
pub enum MirroringType {
    Unknown,
    Horizontal,
    Vertical,
    Both,
}

impl Default for MirroringType {
    fn default() -> MirroringType {
        MirroringType:: Unknown
    }
}