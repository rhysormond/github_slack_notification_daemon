extern crate chrono;
extern crate reqwest;

use std::{env, thread, time};

use chrono::{DateTime, Local};
use reqwest::blocking::{Client, Response};

use crate::github_notification::{GithubNotification, HasHtmlUrl};
use crate::slack_message::SlackMessage;

mod github_notification;
mod slack_message;

/// https://developer.github.com/v3/activity/notifications/
const GITHUB_API_URL: &str = "https://api.github.com/notifications";

const POLLING_FREQUENCY: time::Duration = time::Duration::from_secs(60);

fn main() {
    let client: Client = Client::new();
    let github_username: String = env::var("GITHUB_USERNAME").unwrap();
    let github_token: String = env::var("GITHUB_TOKEN").unwrap();
    let slack_hook: String = env::var("SLACK_HOOK").unwrap();

    let mut last_notification_time: DateTime<Local> = Local::now();

    let initialization_message: SlackMessage =
        SlackMessage::from_initialization_timestamp(last_notification_time);
    client
        .post(&slack_hook)
        .json::<SlackMessage>(&initialization_message)
        .send()
        .unwrap();
    println!("Initialized at {:?}.", last_notification_time);

    loop {
        last_notification_time = Local::now();

        let response: Response = client
            .get(GITHUB_API_URL)
            .basic_auth(&github_username, Some(&github_token))
            .query(&[("since", last_notification_time.to_rfc3339())])
            .send()
            .unwrap();
        println!("Got response: {:?}.", response);

        let notifications: Vec<GithubNotification> =
            response.json::<Vec<GithubNotification>>().unwrap();
        println!("Got notifications: {:?}.", notifications);

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
                println!("Got slack html url {:?}.", with_html_url);
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

        thread::sleep(POLLING_FREQUENCY);
    }
}
