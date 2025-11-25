use std::path::PathBuf;

use phf::{Map, phf_map};
use walkdir::DirEntry;

pub(super) struct Plan {
    pub(super) source: PathBuf,
    pub(super) dest: PathBuf,
}

impl Plan {
    pub(super) fn new(entry: DirEntry) -> Option<Plan> {
        let source = entry.into_path();
        if !source.is_file() {
            return None;
        }

        let ext = source.extension()?.to_str()?;
        let ext = EXT_MAP.get(ext)?;
        let dest = source.with_extension(ext);

        Some(Self { source, dest })
    }
}

static EXT_MAP: Map<&'static str, &'static str> = phf_map! {
    "rpgmvo" => "ogg",
    "rpgmvm" => "m4a",
    "rpgmvp" => "png",
    "ogg_" => "ogg",
    "m4a_" => "m4a",
    "png_" => "png",
};
