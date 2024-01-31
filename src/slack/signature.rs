use faster_hex::hex_string;
use hmac::{Hmac, Mac};
use std::borrow::Cow;
use anyhow::Result;
use sha2::Sha256;

use crate::error::*;

const SLACK_SIGNATURE_VERSION: &str = "v0";

// https://api.slack.com/authentication/verifying-requests-from-slack
pub fn verify_signature<'a, T>(body: T, ts: T, signature: T, key: T) -> Result<()>
  where T: Into<Cow<'a, str>> {

  // Compute the hmac signature
  let base_signature = format!("{}:{}:{}", SLACK_SIGNATURE_VERSION, ts.into(), body.into());
  let mut mac = Hmac::<Sha256>::new_from_slice(key.into().as_bytes())?;
  mac.update(base_signature.as_bytes());

  // Encode the signature and prepend the version
  let encoded_signature = hex_string(mac.finalize().into_bytes().as_ref());
  let encoded_signature = format!("{}={}", SLACK_SIGNATURE_VERSION, encoded_signature);

  if encoded_signature != signature.into() {
    Err(GBError::SlackSignatureFailure.into())
  }
  else {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::slack::headers;

  const TEST_BODY: &str = "token=xyzz0WbapA4vBCDEFasx0q6G&team_id=T1DC2JH3J&team_domain=testteamnow&channel_id=G8PSS9T3V";
  const TEST_SHA_KEY: &str = "keep_it_secret_keep_it_safe";
  const TEST_TIMESTAMP: &str = "1531420618";

  fn test_signature(body: &str, ts: &str, key: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).unwrap();
    mac.update(format!("{}:{}:{}", SLACK_SIGNATURE_VERSION, ts, body).as_bytes());
    format!("{}={}", SLACK_SIGNATURE_VERSION, hex_string(mac.finalize().into_bytes().as_ref()))
  }

  #[test]
  fn test_verify_signature() {
    let signature = test_signature(TEST_BODY, TEST_TIMESTAMP, TEST_SHA_KEY);
    assert!(verify_signature(TEST_BODY, TEST_TIMESTAMP, &signature, TEST_SHA_KEY).is_ok());
  }

  #[test]
  fn test_verify_signature_invalid() {
    let signature = "v0=this is my test signature, there are many others like it but this one is mine";
    assert!(verify_signature(TEST_BODY, TEST_TIMESTAMP, &signature, TEST_SHA_KEY).is_err());
  }

  #[test]
  fn test_verify_signature_wrong_key() {
    let signature = test_signature(TEST_BODY, TEST_TIMESTAMP, TEST_SHA_KEY);
    assert!(verify_signature(TEST_BODY, TEST_TIMESTAMP, &signature, "oops_fake_key").is_err());
  }

  #[test]
  fn test_verify_signature_wrong_timestamp() {
    let signature = test_signature(TEST_BODY, TEST_TIMESTAMP, TEST_SHA_KEY);
    assert!(verify_signature(TEST_BODY, "1531420420", &signature, TEST_SHA_KEY).is_err());
  }
}
