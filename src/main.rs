extern crate chrono;
extern crate reqwest;

use std::{env, thread, time};

use chrono::{DateTime, Local};
use reqwest::blocking::Client;

use crate::github_notification::{GithubNotification, HasHtmlUrl};
use crate::slack_message::SlackMessage;

mod github_notification;
mod slack_message;

/// https://developer.github.com/v3/activity/notifications/
const GITHUB_API_URL: &str = "https://api.github.com/notifications";

const POLLING_FREQUENCY: time::Duration = time::Duration::from_secs(60);

fn main() {
    let client: Client = Client::new();
    let github_username: String =
        env::var("GITHUB_USERNAME").expect("Expected a GITHUB_USERNAME env var.");
    let github_token: String = env::var("GITHUB_TOKEN").expect("Expected a GITHUB_TOKEN env var.");
    let slack_hook: String = env::var("SLACK_HOOK").expect("Expected a SLACK_HOOK env var.");

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

        let notifications: Vec<GithubNotification> = {
            let maybe_response = client
                .get(GITHUB_API_URL)
                .basic_auth(&github_username, Some(&github_token))
                .query(&[("since", last_notification_time.to_rfc3339())])
                .send();

            match maybe_response {
                Ok(response) => {
                    println!("Got response to notification request: {:?}.", response);
                    response.json::<Vec<GithubNotification>>().unwrap()
                }
                Err(error) => panic!("Failed to query the GitHub notification API: {:?}.", error),
            }
        };
        println!("Got notifications: {:?}.", notifications);

        let messages: Vec<SlackMessage> = notifications
            .iter()
            .map(|notification| {
                let maybe_response = client
                    .get(&notification.subject.url)
                    .basic_auth(&github_username, Some(&github_token))
                    .send();

                let with_html_url = match maybe_response {
                    Ok(response) => {
                        println!("Got response to html url request: {:?}.", response);
                        response.json::<HasHtmlUrl>().unwrap()
                    }
                    Err(error) => panic!("Failed to query the GitHub for html url: {:?}.", error),
                };
                SlackMessage::from_github_notification(notification, with_html_url)
            })
            .collect();
        println!("Got slack messages: {:?}.", messages);

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
