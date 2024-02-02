use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Signature {
    pub(crate) signature: String,
    pub(crate) timestamp: String,
    pub(crate) nonce: String,
    #[serde(rename(serialize = "echostr", deserialize = "echostr"))]
    pub(crate) echo_str: String,
}
