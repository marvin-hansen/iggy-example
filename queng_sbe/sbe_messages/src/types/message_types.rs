use std::fmt;
use std::fmt::Display;

/// `MessageType` enum representing different message types.
///
/// Can take following values:
///
/// - `UnknownMessageType` = `0_u16`
///   1xx = `ClientTypes`
/// - `ClientLogin` = `101_u16`
/// - `ClientLogout` = `102_u16`
///   2xx = `DataTypes`
/// - `StartData` = `201_u16`
/// - `StopData` = `202_u16`
/// - `StopAllData` = `203_u16`
/// - `OHLCVBar` = `204_u16`
/// - `FirstOHLCVBar` = `205_u16`
/// - `LastOHLCVBar` = `206_u16`
/// - `TradeBar` = `207_u16`
/// - `FirstTradeBar` = `208_u16`
/// - `LastTradeBar` = `209_u16`
///    8xx = `ErrorTypes`
/// - `ClientError` = `801_u16`
/// - `DataError` = `802_u16`
///
/// # Remarks
///
/// Derives common Rust traits for convenience:
/// - Serialize, Deserialize - serialization
/// - Clone, Copy, Debug, Default - generics
/// - `PartialEq`, Eq - equality
/// - `PartialOrd`, Ord - ordering
/// - Hash - hashability
///
/// Represented as u16 for compactness.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum MessageType {
    #[default]
    UnknownMessageType = 0_u16,
    // Client Message Types
    ClientLogin = 101_u16,
    ClientLogout = 102_u16,
    // Data Message Types
    StartData = 201_u16,
    StopData = 202_u16,
    StopAllData = 203_u16,
    OHLCVBar = 204_u16,
    FirstOHLCVBar = 205_u16,
    LastOHLCVBar = 206_u16,
    TradeBar = 207_u16,
    FirstTradeBar = 208_u16,
    LastTradeBar = 209_u16,
    // Error Message Types
    ClientError = 801_u16,
    DataError = 802_u16,
}

/// Implements `From<u16>` to convert u16 to `MessageType`.
///
/// # Arguments
///
/// * `value` - u16 value to convert
///
/// # Returns
///
/// `MessageType` variant corresponding to u16 value:
///
/// - 0 -> `UnknownMessageType`
/// - 101 -> `ClientLogin`
/// - 102 -> `ClientLogout`
/// - 201 -> `StartData`
/// - 202 -> `StopData`
/// - 203 -> `StopAllData`
/// - 204 -> `OHLCVBar`
/// - 205 -> `FirstOHLCVBar`
/// - 206 -> `LastOHLCVBar`
/// - 207 -> `TradeBar`
/// - 208 -> `FirstTradeBar`
/// - 209 -> `LastTradeBar`
/// - 801 -> `ClientError`
/// - 802 -> `DataError`
/// - Other -> `UnknownMessageType`
///
/// # Remarks
///
/// Allows converting from raw u16 value to `MessageType` enum.
/// Useful when decoding from binary format.
///
impl From<u16> for MessageType {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            0_u16 => Self::UnknownMessageType,
            // Client Message Types
            101_u16 => Self::ClientLogin,
            102_u16 => Self::ClientLogout,
            // Data Message Types
            201_u16 => Self::StartData,
            202_u16 => Self::StopData,
            203_u16 => Self::StopAllData,
            204_u16 => Self::OHLCVBar,
            205_u16 => Self::FirstOHLCVBar,
            206_u16 => Self::LastOHLCVBar,
            207_u16 => Self::TradeBar,
            208_u16 => Self::FirstTradeBar,
            209_u16 => Self::LastTradeBar,
            // Error Message Types
            801_u16 => Self::ClientError,
            802_u16 => Self::DataError,
            _ => Self::UnknownMessageType,
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ClientLogin => write!(f, "ClientLogin"),
            Self::ClientLogout => write!(f, "ClientLogout"),
            Self::StartData => write!(f, "StartData"),
            Self::StopData => write!(f, "StopData"),
            Self::StopAllData => write!(f, "StopAllData"),
            Self::OHLCVBar => write!(f, "OHLCVBar"),
            Self::FirstOHLCVBar => write!(f, "FirstOHLCVBar"),
            Self::LastOHLCVBar => write!(f, "LastOHLCVBar"),
            Self::TradeBar => write!(f, "TradeBar"),
            Self::FirstTradeBar => write!(f, "FirstTradeBar"),
            Self::LastTradeBar => write!(f, "LastTradeBar"),
            Self::UnknownMessageType => write!(f, "UnknownMessageType"),
            Self::ClientError => write!(f, "ClientError"),
            Self::DataError => write!(f, "DataError"),
        }
    }
}
