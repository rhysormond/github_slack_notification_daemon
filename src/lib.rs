//! A library that provides primitives for working with Github's polling API.
//! Here's the most important bits:
//! - [GithubClient](crate::github_client::GithubClient)
//! - [GithubNotification](crate::github_notification::GithubNotification)
//!
//! There are probably other important things, too, but I didn't document those.
pub use github_client::GithubClient;
pub use github_notification::{GithubNotification, HasHtmlUrl, NotificationWithUrl};
pub use slack_client::SlackClient;
pub use slack_message::SlackMessage;

mod github_client;
mod github_notification;
mod slack_client;
mod slack_message;
