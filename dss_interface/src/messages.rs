use phf::phf_map;

pub enum Messages {
    // Error Messages start at 1000
    /// Request to dSS Server returned HTTP Status 200, but 'ok' indicator was 'false'
    E1000 = 1000,
    /// Request to dSS Server failed
    E1001 = 1001,
    // Info Messages start at 3000
    /// Startupmessage
    I3000 = 3000,
    /// Applicaton token successfully retrieved
    I3001 = 3001,
    /// Program ended
    I3002 = 3002,
    /// Connected to dSS Server
    I3003 = 3003,
}
pub static ERRORMESSAGES: phf::Map<i32, &'static str> = phf_map! {
    1000i32 => "Request to dSS Server returned HTTP Status 200, but 'ok' indicator was 'false'",
    1001i32 => "Request to dSS Server failed",
    3000i32 => concat!(env!("CARGO_PKG_NAME")," ", env!("CARGO_PKG_VERSION"), " - startup"),
    3001i32 => "Applicaton token successfully retrieved",
    3002i32 => "Program ended",
    3003i32 => "Connected to dSS Server",
};
