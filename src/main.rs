extern crate chrono;
extern crate reqwest;

use std::{env, thread, time};

use chrono::{DateTime, Local};
use reqwest::blocking::Response;
use reqwest::Error;

use crate::github_client::GithubClient;
use crate::github_notification::{GithubNotification, HasHtmlUrl};
use crate::slack_client::SlackClient;
use crate::slack_message::SlackMessage;

mod github_client;
mod github_notification;
mod slack_client;
mod slack_message;

const POLLING_FREQUENCY: time::Duration = time::Duration::from_secs(30);

fn main() {
    let github_username: String = env::var("GITHUB_USERNAME").expect("No GITHUB_USERNAME env var.");
    let github_token: String = env::var("GITHUB_TOKEN").expect("No GITHUB_TOKEN env var.");
    let slack_hook: String = env::var("SLACK_HOOK").expect("No SLACK_HOOK env var.");

    let github: GithubClient = GithubClient::new(github_username, github_token);
    let slack: SlackClient = SlackClient::new(slack_hook);

    let mut last_fetch_time: DateTime<Local> = Local::now();

    let initialization_message: SlackMessage =
        SlackMessage::new(format!("Initialized at {:?}", last_fetch_time));
    slack.post(&initialization_message).unwrap();

    loop {
        let time_before_fetch: DateTime<Local> = Local::now();
        let maybe_notifications: Result<Response, Error> =
            github.get_notifications(last_fetch_time);
        let notifications: Vec<GithubNotification> = match maybe_notifications {
            Ok(response) => {
                println!("Got response to notification request: {:?}.", response);
                last_fetch_time = time_before_fetch;
                println!("Setting last_fetch_time to {:?}", last_fetch_time);
                response.json::<Vec<GithubNotification>>().unwrap()
            }
            Err(error) => {
                println!("Failed to query the GitHub notification API: {:?}.", error);
                Vec::new()
            }
        };

        let messages: Vec<SlackMessage> = notifications
            .iter()
            .map(|notification| {
                let maybe_with_url: Result<Response, Error> = github.get(&notification.subject.url);
                let with_html_url = match maybe_with_url {
                    Ok(response) => {
                        println!("Got response to html url request: {:?}.", response);
                        response.json::<HasHtmlUrl>().unwrap()
                    }
                    Err(error) => panic!("Failed to query the GitHub for html url: {:?}.", error),
                };
                SlackMessage::from_github_notification(notification, with_html_url)
            })
            .collect();
        println!("Made slack messages: {:?}.", messages);

        for message in messages {
            slack.post(&message).unwrap();
        }

        thread::sleep(POLLING_FREQUENCY);
    }
}
