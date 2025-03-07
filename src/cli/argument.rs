use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
pub(super) struct Argument {
    #[arg(
        short = 'u',
        long,
        help = "upload file to slack. Please set path of a file.",
        value_name = "FILE_PATH"
    )]
    pub upload: Option<String>,

    #[arg(
        short = 's',
        long,
        help = "send a message to slack. Please set message text.",
        value_name = "SEND_MESSAGE"
    )]
    pub send: Option<String>,
}

impl Argument {
    pub fn get() -> Self {
        Argument::parse()
    }
}
