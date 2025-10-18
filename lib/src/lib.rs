use anyhow::{Result, anyhow};

pub struct Decrypter {
    masks: Vec<u8>,
}

impl Decrypter {
    pub fn new(encryption_key: &str) -> Result<Self> {
        let masks = hex::decode(encryption_key) //
            .map_err(|e| anyhow!("Invalid encryptionKey: {e}"))?;

        Ok(Self { masks })
    }

    pub fn decrypt<'a>(&self, bytes: &'a mut [u8]) -> &'a [u8] {
        let body = &mut bytes[16..]; // first 16 bytes are rpg maker's header
        for i in 0..(usize::min(body.len(), self.masks.len())) {
            body[i] ^= self.masks[i];
        }
        body
    }
}
