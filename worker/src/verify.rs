use std::str::FromStr;

use ed25519_dalek::{Signature, VerifyingKey};
use worker::Response;

use crate::RequestHeaders;

const PUBLIC_KEY: &str = "92e0b82a55f2175d7df4130f5f84050b8197f6a5b9e924da4fecdd0872cf60e9";

/// Function to verify the request signature
pub async fn verify_signature(
    headers: &RequestHeaders,
    body: &Vec<u8>,
) -> Result<(), worker::Result<Response>> {
    let public_key_bytes: [u8; 32] = hex::decode(PUBLIC_KEY)
        .map_err(|_| Response::error("A server error occured while decoding key", 500))?
        .try_into()
        .map_err(|_| Response::error("A server error occured while decoding key", 500))?;
    let key = VerifyingKey::from_bytes(&public_key_bytes).map_err(|e| {
        Response::error(
            format!("A server error occured while loading env: {:?}", e),
            500,
        )
    })?;

    let signature = Signature::from_str(headers.signature())
        .map_err(|_| Response::error("Error parsing signature", 500))?;
    let signature_timestamp = headers.timestamp();

    let mut msg = signature_timestamp.as_bytes().to_vec();
    msg.extend_from_slice(&body);

    key.verify_strict(&msg, &signature)
        .map_err(|_| Response::error("Invalid request signature", 403))?;

    Ok(())
}
