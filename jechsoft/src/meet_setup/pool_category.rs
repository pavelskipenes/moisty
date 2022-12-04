use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PoolCategory {
    Meters,
}

#[allow(clippy::recursive_format_impl)]
impl Display for PoolCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => write!(
                f,
                "{}",
                match self {
                    PoolCategory::Meters => "[undocumented] meters",
                }
            ),
        }
    }
}
