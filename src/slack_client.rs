use reqwest::blocking::{Client, Response};
use reqwest::Error;

use crate::slack_message::SlackMessage;

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

    pub fn post(&self, message: &SlackMessage) -> Result<Response, Error> {
        println!("Sending slack message {}", serde_json::json!((message)));
        self.client
            .post(&self.webhook_url)
            .json::<SlackMessage>(message)
            .send()
    }
}
