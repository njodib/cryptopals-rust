use base64_light::{base64_decode, base64_encode}

pub fn bytes_to_base64(bytes: &[u8]) {
    base64_decode(bytes)
}
