# Slack upload CLI

See: [files.upload retires in March 2025, replaced by sequenced Web API methods](https://api.slack.com/changelog/2024-04-a-better-way-to-upload-files-is-here-to-stay)

[files.upload](https://api.slack.com/methods/files.upload) API was replaced to [files.getUploadURLExternal](https://api.slack.com/methods/files.getUploadURLExternal) and [files.completeUploadExternal](https://api.slack.com/methods/files.completeUploadExternal).

However, those new APIs are complex and not easy to use with curl. So, I made this small CLI tool to upload files to Slack.
