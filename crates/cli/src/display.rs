mod decrypt_game_error;
mod invalid_encrypted_bytes_error;
mod parse_encryption_key_error;
mod parse_system_json_error;

use std::fmt;

pub trait AsDisplay<'a> {
    type Target: fmt::Display;
    fn as_display(&'a self) -> Self::Target;
}
