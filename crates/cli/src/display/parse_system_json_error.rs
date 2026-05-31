use std::fmt;

use rpgmvmz_decrypter::ParseSystemJsonError;

use super::AsDisplay;

impl<'a> AsDisplay<'a> for ParseSystemJsonError {
    type Target = ParseSystemJsonErrorDisplay<'a>;
    fn as_display(&'a self) -> Self::Target {
        ParseSystemJsonErrorDisplay(self)
    }
}

pub struct ParseSystemJsonErrorDisplay<'a>(&'a ParseSystemJsonError);

impl fmt::Display for ParseSystemJsonErrorDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ParseSystemJsonError::NotAnObject => {
                write!(f, "content is not an object")
            }
            ParseSystemJsonError::EncryptionKeyNotExists => {
                write!(f, "encryptionKey not exists")
            }
            ParseSystemJsonError::EncryptionKeyIsNotAString => {
                write!(f, "encryptionKey is not a string")
            }
            ParseSystemJsonError::InvalidEncryptionKey { encryption_key, source } => {
                write!(f, "encryptionKey is invalid({encryption_key}): ")?;
                write!(f, "{}", source.as_display())
            }
        }
    }
}
