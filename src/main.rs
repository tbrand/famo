#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate famo_lib;

mod cli;
mod error;
mod phase;

use failure::Error;
use famo_lib::hash;
use famo_lib::lang::detect;
use famo_lib::s3;
use std::env;

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    if let Err(e) = main_inner() {
        error!("{:?}", e);
    }
}

fn main_inner() -> Result<(), Error> {
    let matches = cli::matches();

    let lang = detect(&env::current_dir().unwrap());

    if lang.is_some() {
        info!(
            "Auto detection works! The project is recognized as '{}'.",
            &lang.clone().unwrap().name()
        );
    }

    let archive = cli::archive(&matches, &lang)?;
    debug!("archive={}", archive);

    let watches = cli::watches(&matches, &lang)?;
    debug!("watches={:?}", watches);

    let command = cli::command(&matches, &lang)?;
    debug!("command={}", command);

    let hex = hash::hex(&watches)?;
    debug!("hex={}", hex);

    let verbose = matches.is_present("verbose");
    let async = matches.is_present("async");

    let s3_context = s3::context::Context::from_matches(&matches);
    debug!(
        "endpoint={}, bucket={}, key={:?}",
        s3_context.endpoint().to_owned(),
        s3_context.bucket().to_owned(),
        s3_context.key(),
    );

    let cache_exists = match phase::download_if_cache_exists(&s3_context, &hex) {
        Ok(exists) => exists,
        Err(e) => {
            warn!("{}", e);
            warn!("Failed to download cache from S3.");
            warn!("Continue for the building without cache...");

            false
        }
    };

    phase::execute_command(&command, verbose)?;

    if !cache_exists {
        if async {
            info!("Asyncronous mode. (This function is not working now.)");
        }

        if let Err(e) = phase::upload_archive(&s3_context, &hex, &archive) {
            warn!("{}", e);
            warn!("Failed to upload cache to S3.");
        }
    }

    Ok(())
}
