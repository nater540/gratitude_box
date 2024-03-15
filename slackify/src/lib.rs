pub mod client;
pub mod error;

pub mod prelude {
  pub use crate::client::Client;
  pub use crate::error::SlackifyError;
}
