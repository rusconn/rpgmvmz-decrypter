use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use phf::phf_map;

use crate::Config;

pub(crate) fn copy_with_decryption(config: &Config, source: &Path) -> Result<()> {
    let (dest, do_decrypt) = plan(config, source);

    if let Some(dest_parent) = dest.parent() {
        if !fs::exists(dest_parent)? {
            fs::create_dir_all(dest_parent)?;
        }
    }

    if do_decrypt {
        let bytes = fs::read(source)?;
        fs::write(dest, decrypt(&config.masks, bytes))?;
    } else {
        fs::copy(source, dest)?;
    }

    Ok(())
}

fn plan(config: &Config, source: &Path) -> (PathBuf, bool) {
    let Config { game_dir, dest_root, .. } = config;

    let dest_path = dest_root.join(source.strip_prefix(game_dir).unwrap());
    let ext = source.extension().and_then(|s| s.to_str()).unwrap_or("");
    let dest = dest_path.with_extension(EXT_MAP.get(ext).unwrap_or(&ext));
    let do_decrypt = EXT_MAP.contains_key(ext);

    (dest, do_decrypt)
}

static EXT_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "rpgmvo" => "ogg",
    "rpgmvm" => "m4a",
    "rpgmvp" => "png",
    "ogg_" => "ogg",
    "m4a_" => "m4a",
    "png_" => "png",
};

fn decrypt(masks: &[u8], mut bytes: Vec<u8>) -> Vec<u8> {
    let mut body = bytes.split_off(16); // first 16 bytes are rpg maker's header
    for i in 0..(usize::min(body.len(), masks.len())) {
        body[i] ^= masks[i];
    }
    body
}
