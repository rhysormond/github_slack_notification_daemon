# Github Notifications
A github notification daemon written in Rust.

# Usage
- Generate a github personal access token at: https://github.com/settings/tokens/new?scopes=repo,read:packages,notifications,user&description=Notification+Daemon
- Make a slack incoming webhook following: https://api.slack.com/custom-integrations/incoming-webhooks
- Assign your github username to `$GITHUB_USERNAME`
- Assign the token you just made to `$GITHUB_TOKEN`
- Assign the slack webhook url to the token you just made to `$SLACK_HOOK`
