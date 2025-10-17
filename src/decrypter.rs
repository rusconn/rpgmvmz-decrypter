use anyhow::{Result, anyhow};

pub(crate) struct Decrypter {
    masks: Vec<u8>,
}

impl Decrypter {
    pub(crate) fn new(encryption_key: String) -> Result<Self> {
        let masks = hex::decode(&encryption_key) //
            .map_err(|e| anyhow!("Invalid encryptionKey: {e}"))?;

        Ok(Self { masks })
    }

    pub(crate) fn decrypt<'a>(&self, bytes: &'a mut [u8]) -> &'a [u8] {
        let body = &mut bytes[16..]; // first 16 bytes are rpg maker's header
        for i in 0..(usize::min(body.len(), self.masks.len())) {
            body[i] ^= self.masks[i];
        }
        body
    }
}
