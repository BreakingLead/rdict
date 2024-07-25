mod archive_reader;
mod online_search;

use std::{path::PathBuf, sync::OnceLock};

use archive_reader::{display_word_info, get_word_info};
use clap::Parser;
use colored::Colorize;
use directories::ProjectDirs;
use online_search::get_html;

// struct Config {
//     cache_dir: PathBuf,
// }
//
// fn config() -> &'static Config {
//     static CONFIG: OnceLock<Config> = OnceLock::new();
//
//     CONFIG.get_or_init(|| {
//         let dirs = ProjectDirs::from("", "", "rdict").expect("failed to get cache dir");
//         Config {
//             cache_dir: dirs.cache_dir().to_owned(),
//         }
//     })
// }

/// Arguments to query
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable fzf
    #[arg(short, long)]
    pub fzf: bool,
    /// Word to query
    pub query: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    if let Some(query) = args.query {
        if let Ok(info) = get_word_info(&query.join(" ")) {
            display_word_info(info);
        } else {
            println!("{}", "WORD NOT FOUND".on_red());
        };
    } else {
        println!("bad usage");
    }
}
