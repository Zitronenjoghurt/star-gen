use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn b64_encode_u64(value: u64) -> String {
    STANDARD.encode(value.to_le_bytes())
}

pub fn b64_decode_u64(encoded: &str) -> Option<u64> {
    let decoded = STANDARD.decode(encoded).ok()?;
    let bytes: [u8; 8] = match decoded.try_into() {
        Ok(arr) => arr,
        Err(_) => return None,
    };
    Some(u64::from_le_bytes(bytes))
}
