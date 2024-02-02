#[derive(Debug)]
pub struct MessageResponse<'a> {
    pub(crate) to_username: String,
    pub(crate) from_username: String,
    pub(crate) create_time: u64,
    pub(crate) msg_type: &'a str,
    pub(crate) content: String,
}

impl MessageResponse<'_> {
    pub fn text_message(to_username: String, from_username: String, create_time: u64) -> Self {
        MessageResponse {
            to_username,
            from_username,
            create_time,
            msg_type: "text",
            content: String::new(),
        }
    }
}