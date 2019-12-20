pub use github_client::GithubClient;
pub use github_notification::{GithubNotification, HasHtmlUrl, NotificationWithUrl};
pub use slack_client::SlackClient;
pub use slack_message::{Markdownable, SlackMessage};

mod github_client;
mod github_notification;
mod slack_client;
mod slack_message;
