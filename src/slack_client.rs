use reqwest::Error;
use reqwest::{Client, Response};

use crate::SlackMessage;

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
    pub async fn post<T: Into<String>>(&self, text: T) -> Result<Response, Error> {
        let message = &SlackMessage::new(text.into());
        println!("Sending slack message {}", serde_json::json!(message));
        self.client
            .post(&self.webhook_url)
            .json::<SlackMessage>(&message)
            .send().await
    }
}
