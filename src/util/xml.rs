use quick_xml::de::from_str;

use crate::entity::dto;

pub fn parse_xml(xml_data: &str) -> Result<dto::message::WeChatMessage, quick_xml::DeError> {
    from_str(xml_data)
}

pub fn to_xml(resp: &dto::response::MessageResponse) -> String {
    format!(
        r#"<xml>
            <ToUserName><![CDATA[{}]]></ToUserName>
            <FromUserName><![CDATA[{}]]></FromUserName>
            <CreateTime>{}</CreateTime>
            <MsgType><![CDATA[{}]]></MsgType>
            <Content><![CDATA[{}]]></Content>
            </xml>"#,
        resp.to_username,
        resp.from_username,
        resp.create_time,
        resp.msg_type,
        resp.content
    )
}