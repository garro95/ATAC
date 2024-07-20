use crate::app::files::utils::expand_tilde;
use crate::panic_error;
use clap::builder::Styles;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = Styles::styled())]
pub struct Args {
    /// Main application directory, containing JSON collections files, the atac.toml config file and the atac.log file
    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,

    /// Avoid saving data to the collection file
    #[arg(global = true, long, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    /// Import a collection or request from other file formats (Postman v2.1.0, cURL)
    Import {
        /// The type of file to import
        #[command(subcommand)]
        import_type: ImportType,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ImportType {
    /// Import a Postman v2.1.0 file
    Postman {
        /// Path to the file to import
        import_path: PathBuf,

        /// Max depth at which import should stop creating nested collections and only get the deeper requests
        #[arg(long)]
        max_depth: Option<u16>,
    },

    /// Import a curl file
    Curl {
        /// Path to the file to import
        import_path: PathBuf,

        /// Collection name to save the request to
        collection_name: String,

        /// Request name (will use the file name if none is provided)
        request_name: Option<String>,

        /// Search for deeper files
        #[arg(short, long, conflicts_with = "request_name")]
        recursive: bool,

        /// Max depth at which import should stop creating nested collections and only get the deeper requests
        #[arg(long, requires = "recursive", conflicts_with = "request_name")]
        max_depth: Option<u16>,
    },
}

pub struct ParsedArgs {
    pub directory: PathBuf,
    #[allow(dead_code)]
    pub is_directory_from_env: bool,
    pub command: Option<Command>,
    pub should_save: bool,
}

lazy_static! {
    pub static ref ARGS: ParsedArgs = {
        let args = Args::parse();

        let (directory, is_directory_from_env) = match args.directory {
            // If a directory was provided with a CLI argument
            Some(arg_directory) => (expand_tilde(arg_directory), false),
            // If no directory was provided with the CLI
            None => match env::var("ATAC_MAIN_DIR") {
                // If the ATAC_MAIN_DIR environment variable exists
                Ok(env_directory) => (expand_tilde(PathBuf::from(env_directory)), true),
                Err(_) => panic_error("No directory provided, provide one either with `--directory <dir>` or via the environment variable `ATAC_MAIN_DIR`")
            }
        };

        ParsedArgs {
            directory,
            is_directory_from_env,
            command: args.command,
            should_save: !args.dry_run
        }
    };
}
