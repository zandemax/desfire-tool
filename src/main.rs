#[macro_use] extern crate text_io;
extern crate libc;

use quicli::prelude::*;
use structopt::StructOpt;

mod desfire;
mod commands;

fn main() -> CliResult {
    let args: DesfireTool = DesfireTool::from_args();

    match args {
        DesfireTool::Personalize                                => commands::personalize(),
        DesfireTool::Key { key_command, aid }                   => commands::key(key_command, aid),
        DesfireTool::Application { application_command}         => commands::application(application_command),
        DesfireTool::File { file_command, aid, fid }            => commands::file(file_command, aid, fid),
        DesfireTool::Card { card_command }                      => commands::card(card_command),
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "desfire-tool", about = "Tool to interact with DesFire Cards", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
enum DesfireTool {
    #[structopt(name = "personalize", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Personalize the Card. This sets initial configuration like the PICC Key and PICC Config.
    Personalize,
    #[structopt(name = "key", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Change Key settings, add or remove keys
    Key {
        #[structopt(subcommand)]
        key_command: KeyCommand,
        #[structopt(short = "aid", long = "appid", help = "Application ID to perform operation for")]
        aid: u8
    },
    #[structopt(name = "application", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Configure, Add or Remove Applications
    Application {
        #[structopt(subcommand)]
        application_command: ApplicationCommand,
    },
    #[structopt(name = "file", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Configure, Add or Remove Applications
    File {
        #[structopt(subcommand)]
        file_command: FileCommand,
        #[structopt(short = "aid", long = "appid", help = "Application ID the file belongs to")]
        aid: u8,
        #[structopt(short = "fid", long = "fileid", help = "File ID to perform operation for")]
        fid: u8
    },
    #[structopt(name = "card", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Card wide (PICC) Settings and Operations
    Card {
        #[structopt(subcommand)]
        card_command: CardCommand
    }
}

#[derive(StructOpt)]
pub enum KeyCommand {
    #[structopt(name = "add")]
    /// Add a new key for selected Application ID
    Add {
        #[structopt(short = "kid", long = "keyid", help = "Key ID for the newly added key")]
        kid: u8
    },
    #[structopt(name = "configure")]
    /// Configure the KeySettings for the specified Key ID
    Configure {
        #[structopt(short = "kid", long = "keyid", help = "Key ID to perform operation for")]
        kid: u8
    },
    #[structopt(name = "change")]
    /// Change the Key with the specified ID to a new one
    Change {
        #[structopt(short = "kid", long = "keyid", help = "Key ID to perform operation for")]
        kid: u8
    },
    #[structopt(name = "remove")]
    /// Remove the key with the specified Key ID
    Remove {
        #[structopt(short = "kid", long = "keyid", help = "Key ID of the Key to remove")]
        kid: u8,
    },
    #[structopt(name = "info")]
    /// Display Info about the specified Key
    Info {
        #[structopt(short = "kid", long = "keyid", help = "Key ID of the Key to display info for")]
        kid: u8,
    },
    #[structopt(name = "list")]
    /// List all keys for selected Application ID
    List
}

#[derive(StructOpt)]
pub enum FileCommand {
    #[structopt(name = "add")]
    Add {

    },
    #[structopt(name = "configure")]
    Configure {

    },
    #[structopt(name = "change")]
    Change {

    },
    #[structopt(name = "remove")]
    Remove {

    },
    #[structopt(name = "info")]
    /// Show FileSettings and Type
    Info {

    },
    #[structopt(name = "list")]
    List {
        #[structopt(short = "i", long = "iso")]
        /// Print file IDs in ISO Format
        iso: bool
    }
}

#[derive(StructOpt)]
pub enum ApplicationCommand {
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "aid", long = "appid", help = "App ID to perform operation for")]
        aid: u8
    },
    #[structopt(name = "configure")]
    /// Configure the KeySettings for the specified Key ID
    Configure {
        #[structopt(short = "aid", long = "appid", help = "App ID to perform operation for")]
        aid: u8
    },
    #[structopt(name = "change")]
    /// Change the Key with the specified ID to a new one
    Change {
        #[structopt(short = "aid", long = "appid", help = "App ID to perform operation for")]
        aid: u8
    },
    #[structopt(name = "remove")]
    /// Remove the key with the specified Key ID
    Remove {
        #[structopt(short = "aid", long = "appid", help = "App ID of the App to remove")]
        aid: u8,
    },
    #[structopt(name = "info")]
    /// Display Info about the specified Key
    Info {
        #[structopt(short = "aid", long = "appid", help = "App ID of the App to display info for")]
        aid: u8,
    },
    #[structopt(name = "list")]
    /// List all keys for selected Application ID
    List
}

#[derive(StructOpt)]
pub enum CardCommand {
    #[structopt(name = "info")]
    /// Show Infos about your card (config, free space)
    Info,
    #[structopt(name = "format")]
    /// Format your Card (CAUTION: This deletes everything on your card!)
    Format,
    #[structopt(name = "configure")]
    /// Set PICC Settings
    Configure,
}