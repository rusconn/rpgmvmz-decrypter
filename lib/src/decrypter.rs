use crate::encryption_key::EncryptionKey;

pub fn decrypt<'a>(bytes: &'a mut [u8], encryption_key: &EncryptionKey) -> &'a [u8] {
    let body = &mut bytes[16..]; // first 16 bytes are rpg maker's header
    for (bit, mask) in Iterator::zip(body.iter_mut(), encryption_key.iter()) {
        *bit ^= mask;
    }
    body
}
