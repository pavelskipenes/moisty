use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum PoolLength {
    /// 25 meters pool. Often called "short course".
    #[serde(rename = "25")]
    PoolLength25,
    /// 50 meters pool. Often called "long course".
    #[serde(rename = "50")]
    PoolLength50,
}
