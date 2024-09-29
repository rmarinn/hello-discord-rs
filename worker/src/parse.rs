use worker::{Request, Response};

pub struct RequestHeaders {
    signature: String,
    timestamp: String,
}

impl RequestHeaders {
    pub fn signature(&self) -> &str {
        &self.signature
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}

pub fn parse_headers(req: &Request) -> Result<RequestHeaders, worker::Result<Response>> {
    let signature = req
        .headers()
        .get("X-Signature-Ed25519")
        .unwrap()
        .ok_or(Response::error(
            "Missing signature header: X-Signature-Ed25519",
            400,
        ))?;

    let timestamp = req
        .headers()
        .get("X-Signature-Timestamp")
        .unwrap()
        .ok_or(Response::error(
            "Missing signature header: X-Signature-Timestamp",
            400,
        ))?;

    Ok(RequestHeaders {
        signature,
        timestamp,
    })
}

pub async fn parse_body(req: &mut Request) -> Result<Vec<u8>, worker::Result<Response>> {
    let body = req
        .bytes()
        .await
        .map_err(|_| Response::error("Failed to read request body", 500))?;
    Ok(body)
}
