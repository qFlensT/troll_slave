use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct OsInfo{
	pub os_type: String,
	pub version: String,
	pub edition: String,
	pub codename: String,
	pub bitness: String,
	pub hostname: String,
}