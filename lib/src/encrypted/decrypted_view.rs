pub struct DecryptedView(pub(super) Vec<u8>);

use super::ENCRYPTION_HEADER;

impl DecryptedView {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[ENCRYPTION_HEADER.len()..]
    }
}
