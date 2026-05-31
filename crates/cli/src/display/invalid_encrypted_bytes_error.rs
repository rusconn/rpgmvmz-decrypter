use std::fmt;

use rpgmvmz_decrypter::InvalidEncryptedBytesError;

use super::AsDisplay;

impl<'a> AsDisplay<'a> for InvalidEncryptedBytesError {
    type Target = InvalidEncryptedBytesErrorDisplay<'a>;
    fn as_display(&'a self) -> Self::Target {
        InvalidEncryptedBytesErrorDisplay(self)
    }
}

pub struct InvalidEncryptedBytesErrorDisplay<'a>(&'a InvalidEncryptedBytesError);

impl fmt::Display for InvalidEncryptedBytesErrorDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            InvalidEncryptedBytesError::InvalidEncryptionHeader => {
                write!(f, "invalid encryption header")
            }
        }
    }
}
