extern crate reqwest;

use std::collections::HashMap;
use std::env;

use reqwest::blocking::{Client, Response};

use crate::github_notification::{GithubNotification, HasHtmlUrl};
use crate::slack_message::SlackMessage;

mod github_notification;
mod slack_message;

/// https://developer.github.com/v3/activity/notifications/
const GITHUB_API_URL: &str = "https://api.github.com/notifications";

fn main() {
    let client: Client = Client::new();
    let github_username: String = env::var("GITHUB_USERNAME").unwrap();
    let github_token: String = env::var("GITHUB_TOKEN").unwrap();
    let slack_hook: String = env::var("SLACK_HOOK").unwrap();

    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("all", "true");

    let response: Response = client
        .get(GITHUB_API_URL)
        .basic_auth(&github_username, Some(&github_token))
        .json(&params)
        .send()
        .unwrap();

    let notifications: Vec<GithubNotification> =
        response.json::<Vec<GithubNotification>>().unwrap();

    let messages: Vec<SlackMessage> = notifications
        .iter()
        .map(|notification| {
            let with_html_url = client
                .get(&notification.subject.url)
                .basic_auth(&github_username, Some(&github_token))
                .send()
                .unwrap()
                .json::<HasHtmlUrl>()
                .unwrap();
            SlackMessage::from_github_notification(notification, with_html_url)
        })
        .collect();

    messages.iter().for_each(|payload| {
        println!("Sending slack message {}", serde_json::json!((payload)));
        client
            .post(&slack_hook)
            .json::<SlackMessage>(payload)
            .send()
            .unwrap();
        println!("Sent slack message.");
    });
}
