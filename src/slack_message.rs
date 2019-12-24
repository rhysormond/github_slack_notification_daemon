use serde::Serialize;

/// A Slack message formatted for their incoming webhook API
///
/// https://api.slack.com/block-kit
///
/// TODO: add support for more block types
///
/// # Arguments
///
/// * `blocks` - Individual components (e.g. sections with text, images, etc.) that form the body of the message
/// * `icon_emoji` - A slack-formatted emoji name that should be used in place of an avatar
/// * `username` - The username associated with the post
#[derive(Serialize, Debug)]
pub struct SlackMessage {
    blocks: Vec<SectionBlock>,
    icon_emoji: String,
    username: String,
}

impl SlackMessage {
    pub fn new(text: String) -> Self {
        SlackMessage {
            blocks: vec![SectionBlock::new(text)],
            icon_emoji: ":chart_with_upwards_trend:".to_string(),
            username: "Github Notification Daemon".to_string(),
        }
    }
}

/// A generic block component that can wrap text, images, or other accessory fields
///
/// # Arguments
///
/// * `kind` - The type of component that determines how its other fields should be parsed
/// * `text` - A text field to be rendered as markdown
#[derive(Serialize, Debug)]
struct SectionBlock {
    #[serde(rename = "type")]
    kind: String,
    text: TextField,
}

impl SectionBlock {
    fn new(text: String) -> Self {
        Self {
            kind: String::from("section"),
            text: TextField::new(text),
        }
    }
}

/// A field of a section block that contains markdown-formatted text
///
/// # Arguments
///
/// * `kind` - The type of component that determines how its other fields should be parsed
/// * `text` - The markdown-formatted text to be rendered in the section
#[derive(Serialize, Debug)]
struct TextField {
    #[serde(rename = "type")]
    kind: String,
    text: String,
}

impl TextField {
    fn new(text: String) -> Self {
        Self {
            kind: String::from("mrkdwn"),
            text,
        }
    }
}
