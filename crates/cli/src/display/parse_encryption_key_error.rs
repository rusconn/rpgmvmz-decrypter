use std::fmt;

use rpgmvmz_decrypter::ParseEncryptionKeyError;

use super::AsDisplay;

impl<'a> AsDisplay<'a> for ParseEncryptionKeyError {
    type Target = ParseEncryptionKeyErrorDisplay<'a>;
    fn as_display(&'a self) -> Self::Target {
        ParseEncryptionKeyErrorDisplay(self)
    }
}

pub struct ParseEncryptionKeyErrorDisplay<'a>(&'a ParseEncryptionKeyError);

impl fmt::Display for ParseEncryptionKeyErrorDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ParseEncryptionKeyError::InvalidCharacter { c, index } => {
                write!(f, "invalid character '{}' at position {}", c, index + 1)
            }
            ParseEncryptionKeyError::InvalidLength => {
                write!(f, "invalid length")
            }
        }
    }
}
