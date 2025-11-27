use thiserror::Error;

use rpgmvmz_decrypter::filesystem;

#[derive(Debug, Error)]
pub(crate) enum AppError {
    #[error("{}", show_decryption_error(.0))]
    FileSystemDecryption(#[from] filesystem::DecryptionError),
}

fn show_decryption_error(e: &filesystem::DecryptionError) -> String {
    match e {
        filesystem::DecryptionError::NotExists(path) => {
            format!("specified path does not exist: {}", path.display())
        }
        filesystem::DecryptionError::NotADirectory(path) => {
            format!("specified path is not a directory: {}", path.display())
        }
        filesystem::DecryptionError::SystemJsonNotFound => "System.json was not found".into(),
        filesystem::DecryptionError::ReadSystemJson { path, source } => {
            format!("failed to read System.json({}): {source}", path.display())
        }
        filesystem::DecryptionError::ParseSystemJson { path, source } => {
            format!(
                "failed to parse System.json({}): {}",
                path.display(),
                show_parse_system_json_error(source)
            )
        }
        filesystem::DecryptionError::Scan { path, source } => match path {
            Some(path) => {
                format!("failed to scan {}: {source}", path.display())
            }
            _ => format!("failed to scan: {source}"),
        },
        filesystem::DecryptionError::ReadEncryptedFile { path, source } => {
            format!(
                "failed to read encrypted file({}): {source}",
                path.display()
            )
        }
        filesystem::DecryptionError::WriteDecryptedFile { path, source } => {
            format!(
                "failed to write decrypted file({}): {source}",
                path.display()
            )
        }
        filesystem::DecryptionError::RemoveEncryptedFile { path, source } => {
            format!(
                "failed to remove encrypted file({}): {source}",
                path.display()
            )
        }
        filesystem::DecryptionError::MarkSystemJsonAsUnencrypted { path, source } => {
            format!(
                "failed to mark System.json as unencrypted({}): {source}",
                path.display()
            )
        }
    }
}

fn show_parse_system_json_error(e: &filesystem::ParseSystemJsonError) -> String {
    match e {
        filesystem::ParseSystemJsonError::NotAnObject => "content is not an object".into(),
        filesystem::ParseSystemJsonError::EncryptionKeyNotExists => {
            "encryptionKey not exists".into()
        }
        filesystem::ParseSystemJsonError::EncryptionKeyIsNotAString => {
            "encryptionKey is not a string".into()
        }
        filesystem::ParseSystemJsonError::InvalidEncryptionKey { encryption_key, source } => {
            format!(
                "encryptionKey is invalid({encryption_key}): {}",
                show_invalid_encryption_key_error(source)
            )
        }
    }
}

fn show_invalid_encryption_key_error(e: &filesystem::InvalidEncryptionKeyError) -> String {
    match e {
        filesystem::InvalidEncryptionKeyError::InvalidCharacter { c, index } => {
            format!("invalid character '{}' at position {}", c, index + 1)
        }
        filesystem::InvalidEncryptionKeyError::InvalidLength => "invalid length".into(),
    }
}
