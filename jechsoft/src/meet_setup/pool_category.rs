extern crate serde;
use self::serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum PoolCategory {
    Meters,
}
