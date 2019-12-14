use reqwest::blocking::{Client, Response};
use reqwest::Error;

use crate::slack_message::SlackMessage;

/// A client to send messages to the slack incoming webhook API
///
/// # Arguments
///
/// * `client` - The reqwests client that posts to the slack API
/// * `webhook_url` - A slack incoming webhook URL
pub struct SlackClient {
    client: Client,
    webhook_url: String,
}

impl SlackClient {
    pub fn new(webhook_url: String) -> Self {
        Self {
            client: Client::new(),
            webhook_url,
        }
    }

    /// Posts a message to the slack incoming webhook API
    pub fn post(&self, message: &SlackMessage) -> Result<Response, Error> {
        println!("Sending slack message {}", serde_json::json!((message)));
        self.client
            .post(&self.webhook_url)
            .json::<SlackMessage>(message)
            .send()
    }
}
