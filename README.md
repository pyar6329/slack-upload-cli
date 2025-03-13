# Slack upload CLI

See: [files.upload retires in March 2025, replaced by sequenced Web API methods](https://api.slack.com/changelog/2024-04-a-better-way-to-upload-files-is-here-to-stay)

[files.upload](https://api.slack.com/methods/files.upload) API was replaced to [files.getUploadURLExternal](https://api.slack.com/methods/files.getUploadURLExternal) and [files.completeUploadExternal](https://api.slack.com/methods/files.completeUploadExternal).

However, those new APIs are complex and not easy to use with curl. So, I made this small CLI tool to upload files to Slack.

# Download

```bash
$ curl -sL -o suc https://github.com/pyar6329/slack-upload-cli/releases/download/1.0.1/suc-$(uname -s)-$(uname -m)
$ chmod +x suc
$ ./suc --help
```

Please move suc command to PATH directory.

# Usage

```bash
Usage: suc [OPTIONS]

Options:
  -u, --upload <FILE_PATH>   upload file to slack. Please set path of a file.
  -s, --send <SEND_MESSAGE>  send a message to slack. Please set message text.
  -h, --help                 Print help
  -V, --version              Print version
```

## Environment Variables

please set below environment variables

```bash
export SLACK_API_TOKEN="xoxb-xxxxxxxxxxxx-xxxxxxxxxxxx-"
export SLACK_CHANNEL_ID="C12345678"
```

## Upload a file

run the command

```bash
$ suc -u /path/to/file
```

## Send a message

run the command

```bash
$ suc -s "Hello, World!"
```
