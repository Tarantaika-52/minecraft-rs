use std::path::Path;

use crate::{internal_types::shared::VersionJsonLibrary, utils};

pub fn get_natives(data: &VersionJsonLibrary) -> &str {
    let arch_type = match utils::get_arch() {
        "x86_64" => "",
        v => v,
    };

    // TODO!

    ""
}

pub async fn extract_native(from: &Path, to: &Path) {
    //
}
