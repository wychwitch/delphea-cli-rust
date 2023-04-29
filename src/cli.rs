const APP_INFO: AppInfo = AppInfo {
    name: "Obsidian cli quick-logger",
    author: "wych(witch) <wych@wychwit.ch>",
};

#[derive(clap::Parser)]
#[command(name = APP_INFO.name)]
#[command(author = APP_INFO.author)]
#[command(version = "0.1.0")]
#[command(about = "Quickly sends things to obsidian over cli", long_about = None, arg_required_else_help = true, after_help = "NOTE: Target is relative to root. Must not begin or end with a slash. Can accept periodic note specifications instead such as daily or quarterly.")]
struct Args {
    #[command(subcommand)]
    action: Action,
}
//an enum of all available commands
#[derive(clap::Subcommand, Debug)]
enum Action {
    /// Log to a specified file. USAGE: obs log <TEXT>
    ///
    /// Add 
    Add {sheet: String, body: String },
    /// set the api-key USAGE: obs key <API_KEY>
    Key { api_key: String },
    ///Change the target file. USAGE obs target <TARGET_FILE>
    Target { target_file: String },
    ///retrieves the currently set target file
    GetTarget,
}
