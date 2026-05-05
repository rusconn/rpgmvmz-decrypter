use thiserror::Error;

use rpgmvmz_decrypter::{
    DecryptGameError, InvalidEncryptedBytesError, ParseEncryptionKeyError, ParseSystemJsonError,
};

#[derive(Debug, Error)]
pub(crate) enum AppError {
    #[error("{}", show_decrypt_game_error(.0))]
    FileSystemDecryption(#[from] DecryptGameError),
}

fn show_decrypt_game_error(e: &DecryptGameError) -> String {
    match e {
        DecryptGameError::NotExists(path) => {
            format!("specified path does not exist: {}", path.display())
        }
        DecryptGameError::NotADirectory(path) => {
            format!("specified path is not a directory: {}", path.display())
        }
        DecryptGameError::SystemJsonNotFound => "System.json was not found".into(),
        DecryptGameError::ReadSystemJson { path, source } => {
            format!("failed to read System.json({}): {source}", path.display())
        }
        DecryptGameError::ParseSystemJson { path, source } => {
            format!(
                "failed to parse System.json({}): {}",
                path.display(),
                show_parse_system_json_error(source)
            )
        }
        DecryptGameError::Scan { path, source } => match path {
            Some(path) => {
                format!("failed to scan {}: {source}", path.display())
            }
            _ => format!("failed to scan: {source}"),
        },
        DecryptGameError::ReadEncryptedFile { path, source } => {
            format!(
                "failed to read encrypted file({}): {source}",
                path.display()
            )
        }
        DecryptGameError::InvalidEncryptedFile { path, source } => {
            format!(
                "failed to decrypt encrypted file({}): {}",
                path.display(),
                show_invalid_encrypted_bytes_error(source)
            )
        }
        DecryptGameError::WriteDecryptedFile { path, source } => {
            format!(
                "failed to write decrypted file({}): {source}",
                path.display()
            )
        }
        DecryptGameError::RemoveEncryptedFile { path, source } => {
            format!(
                "failed to remove encrypted file({}): {source}",
                path.display()
            )
        }
        DecryptGameError::MarkSystemJsonAsUnencrypted { path, source } => {
            format!(
                "failed to mark System.json as unencrypted({}): {source}",
                path.display()
            )
        }
    }
}

fn show_parse_system_json_error(e: &ParseSystemJsonError) -> String {
    match e {
        ParseSystemJsonError::NotAnObject => "content is not an object".into(),
        ParseSystemJsonError::EncryptionKeyNotExists => "encryptionKey not exists".into(),
        ParseSystemJsonError::EncryptionKeyIsNotAString => "encryptionKey is not a string".into(),
        ParseSystemJsonError::InvalidEncryptionKey { encryption_key, source } => {
            format!(
                "encryptionKey is invalid({encryption_key}): {}",
                show_invalid_encryption_key_error(source)
            )
        }
    }
}

fn show_invalid_encryption_key_error(e: &ParseEncryptionKeyError) -> String {
    match e {
        ParseEncryptionKeyError::InvalidCharacter { c, index } => {
            format!("invalid character '{}' at position {}", c, index + 1)
        }
        ParseEncryptionKeyError::InvalidLength => "invalid length".into(),
    }
}

fn show_invalid_encrypted_bytes_error(e: &InvalidEncryptedBytesError) -> String {
    match e {
        InvalidEncryptedBytesError::InvalidEncryptionHeader => "invalid encryption header".into(),
    }
}
