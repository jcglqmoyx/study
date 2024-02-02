use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeChatMessage {
    #[serde(rename(serialize = "ToUserName", deserialize = "ToUserName"))]
    pub(crate) to_user_name: String,

    #[serde(rename(serialize = "FromUserName", deserialize = "FromUserName"))]
    pub(crate) from_user_name: String,

    #[serde(rename(serialize = "CreateTime", deserialize = "CreateTime"))]
    pub(crate) create_time: u64,

    #[serde(rename(serialize = "MsgType", deserialize = "MsgType"))]
    pub(crate) msg_type: String,

    #[serde(rename(serialize = "Content", deserialize = "Content"))]
    pub(crate) content: Option<String>,
    #[serde(rename(serialize = "PicUrl", deserialize = "PicUrl"))]
    pub(crate) pic_url: Option<String>,

    /*
    #[serde(rename(serialize = "MsgId", deserialize = "MsgId"))]
    msg_id: u64,
    #[serde(rename(serialize = "MediaId", deserialize = "MediaId"))]
    pub(crate) media_id: Option<String>,
    #[serde(rename(serialize = "MsgDataId", deserialize = "MsgDataId"))]
    pub(crate) msg_data_id: Option<String>,
    #[serde(rename(serialize = "Idx", deserialize = "Idx"))]
    pub(crate) idx: Option<String>,
    #[serde(rename(serialize = "Format", deserialize = "Format"))]
    pub(crate) format: Option<String>,
    #[serde(rename(serialize = "ThumbMediaId", deserialize = "ThumbMediaId"))]
    pub(crate) thumb_media_id: Option<String>,
     */
}

