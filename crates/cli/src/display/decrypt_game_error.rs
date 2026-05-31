use std::fmt;

use rpgmvmz_decrypter::DecryptGameError;

use super::AsDisplay;

impl<'a> AsDisplay<'a> for DecryptGameError {
    type Target = DecryptGameErrorDisplay<'a>;
    fn as_display(&'a self) -> Self::Target {
        DecryptGameErrorDisplay(self)
    }
}

pub struct DecryptGameErrorDisplay<'a>(&'a DecryptGameError);

impl fmt::Display for DecryptGameErrorDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            DecryptGameError::PathNotExists(path) => {
                write!(f, "specified path does not exist: {}", path.display())
            }
            DecryptGameError::PathIsNotADirectory(path) => {
                write!(f, "specified path is not a directory: {}", path.display())
            }
            DecryptGameError::SystemJsonNotFound => {
                write!(f, "System.json was not found")
            }
            DecryptGameError::ReadSystemJson { path, source } => {
                write!(
                    f,
                    "failed to read System.json({}): {source}",
                    path.display()
                )
            }
            DecryptGameError::ParseSystemJson { path, source } => {
                write!(f, "failed to parse System.json({}): ", path.display())?;
                write!(f, "{}", source.as_display())
            }
            DecryptGameError::ScanDirectory { path, source } => match path {
                Some(path) => {
                    write!(f, "failed to scan {}: {source}", path.display())
                }
                _ => {
                    write!(f, "failed to scan: {source}")
                }
            },
            DecryptGameError::ReadEncryptedFile { path, source } => {
                write!(
                    f,
                    "failed to read encrypted file({}): {source}",
                    path.display()
                )
            }
            DecryptGameError::InvalidEncryptedFile { path, source } => {
                write!(f, "failed to decrypt encrypted file({}): ", path.display())?;
                write!(f, "{}", source.as_display())
            }
            DecryptGameError::WriteDecryptedFile { path, source } => {
                write!(
                    f,
                    "failed to write decrypted file({}): {source}",
                    path.display()
                )
            }
            DecryptGameError::RemoveEncryptedFile { path, source } => {
                write!(
                    f,
                    "failed to remove encrypted file({}): {source}",
                    path.display()
                )
            }
            DecryptGameError::MarkSystemJsonAsUnencrypted { path, source } => {
                write!(
                    f,
                    "failed to mark System.json as unencrypted({}): {source}",
                    path.display()
                )
            }
        }
    }
}
