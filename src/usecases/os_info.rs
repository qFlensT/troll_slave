use crate::models::os_info::OsInfo;
use os_info as oinfo;

pub fn os_info() -> OsInfo {
    let info = oinfo::get();

    OsInfo {
        os_type: info.os_type().to_string(),
        version: info.version().to_string(),
        codename: info.codename().unwrap_or_default().to_string(),
        bitness: info.bitness().to_string(),
        edition: info.edition().unwrap_or_default().to_string(),
        hostname: gethostname::gethostname().to_string_lossy().to_string(),
    }
}
