pub mod signature;
pub mod messages;
pub mod client;
pub mod http;
pub mod verification_middleware;

pub use http::headers;
pub use verification_middleware::VerificationMiddleware;

// use serde::{Deserialize, Deserializer};
// use chrono::{DateTime, Utc, TimeZone};
// use std::str::FromStr;

// pub fn slack_timestamp_deserializer<'de, D>(deserializer: D) -> anyhow::Result<DateTime<Utc>>
//   where D: Deserializer<'de> {
//     let s = String::deserialize(deserializer)?;
//     let parts: Vec<&str> = s.split('.').collect();

//     if parts.len() == 2 {
//       let seconds = i64::from_str(parts[0]).map_err(serde::de::Error::custom)?;
//       let nanoseconds = u32::from_str(parts[1]).map_err(serde::de::Error::custom)? * 1_000;

//       match Utc.timestamp_opt(seconds, nanoseconds) {
//         chrono::LocalResult::Single(datetime) => Ok(datetime),
//         _ => Err(serde::de::Error::custom("Invalid timestamp")),
//       }
//     } else {
//       Err(serde::de::Error::custom("Invalid timestamp format"))
//     }
// }
