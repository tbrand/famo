use clap::{App, Arg, ArgMatches};
use error::FamoError;
use failure::Error;
use famo_lib::lang::Lang;

pub fn matches<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("An Engine for Caching Builds on CI/CD")
        .arg(arg_access_key_id())
        .arg(arg_secret_access_key())
        .arg(arg_bucket())
        .arg(arg_endpoint())
        .arg(arg_region())
        .arg(arg_key())
        .arg(arg_archive())
        .arg(arg_command())
        .arg(arg_verbose())
        .arg(arg_async())
        .arg(arg_watch())
        .get_matches()
}

pub fn archive<'a>(matches: &'a ArgMatches, lang: &Option<Lang>) -> Result<&'a str, Error> {
    if let Some(archive) = matches.value_of("archive") {
        return Ok(archive);
    }

    if let Some(lang) = lang {
        return Ok(lang.build());
    }

    Err(FamoError::MissedOption {
        description: "Archived directory is not specified. (--archive=[directory])\n".to_owned()
            + "You can see help messages by 'famo -h'",
    }.into())
}

pub fn watches<'a>(matches: &'a ArgMatches, lang: &Option<Lang>) -> Result<Vec<&'a str>, Error> {
    if let Some(watches) = matches.values_of("watch") {
        return Ok(watches.collect::<Vec<&str>>());
    }

    if let Some(lang) = lang {
        return Ok(lang.watches().to_owned());
    }

    Err(FamoError::MissedOption {
        description: "Watched files are not specified.\n".to_owned()
            + "You can see help messages by 'famo -h'",
    }.into())
}

pub fn command<'a>(matches: &'a ArgMatches, lang: &Option<Lang>) -> Result<&'a str, Error> {
    if let Some(command) = matches.value_of("command") {
        return Ok(command);
    }

    if let Some(lang) = lang {
        return Ok(lang.command());
    }

    Err(FamoError::MissedOption {
        description: "Build command is not specified. (--command=\"build command\")\n".to_owned()
            + "You can see help messages by 'famo -h'",
    }.into())
}

fn arg_access_key_id<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("access_key_id")
        .help("Access Key ID for S3 uploads/downloads")
        .takes_value(true)
        .long("access_key_id")
        .env("FAMO_ACCESS_KEY_ID")
        .required(true)
        .hide_env_values(true)
}

fn arg_secret_access_key<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("secret_access_key")
        .help("Secret Access Key for S3 uploads/downloads")
        .takes_value(true)
        .long("secret_access_key")
        .env("FAMO_SECRET_ACCESS_KEY")
        .required(true)
        .hide_env_values(true)
}

fn arg_bucket<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("bucket")
        .help("A bucket for storing caches")
        .takes_value(true)
        .long("bucket")
        .short("b")
        .env("FAMO_BUCKET")
        .required(true)
}

fn arg_endpoint<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("endpoint")
        .help("An endpoint for uploading/downloading caches")
        .takes_value(true)
        .long("endpoint")
        .short("e")
        .env("FAMO_ENDPOINT")
        .required(true)
}

fn arg_region<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("region")
        .help("A region of the endpoint")
        .takes_value(true)
        .long("region")
        .short("r")
        .env("FAMO_REGION")
        .required(true)
}

fn arg_key<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("key")
        .help("Optional key of the object on S3. ({bucket}/{key}/{cache hex})")
        .takes_value(true)
        .long("key")
        .short("k")
        .env("FAMO_KEY")
        .required(false)
}

fn arg_archive<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("archive")
        .help("Target directory to be cached")
        .takes_value(true)
        .long("archive")
        .short("a")
        .env("FAMO_ARCHIVE")
}

fn arg_watch<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("watch")
        .help("Paths of target files to be watched")
        .multiple(true)
}

fn arg_command<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("command")
        .help("Build command of the project")
        .takes_value(true)
        .long("command")
        .short("c")
}

fn arg_verbose<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("verbose")
        .help("Show output of the command")
        .takes_value(false)
        .long("--verbose")
        .short("-v")
}

fn arg_async<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("async")
        .help("Uploading archive asyncronously. Use `famo wait` to sync the process.")
        .takes_value(false)
        .long("--async")
}
