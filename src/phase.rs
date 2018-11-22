use failure::Error;
use famo_lib::archive;
use famo_lib::s3;
use famo_lib::s3::context::Context as S3Context;
use std::process::{Command, Stdio};

pub fn download_if_cache_exists(s3_context: &S3Context, hex: &str) -> Result<bool, Error> {
    if s3::key_exists(&s3_context, &hex)? {
        info!("The cache exists on S3.");

        info!("--- Downloading");
        let bytes = s3::download(&s3_context, &hex)?;
        info!("--- ---> Done ({} bytes)", bytes.len());

        info!("--- Decoding");
        let bytes = archive::decode(bytes.as_slice(), Vec::new())?;
        info!("--- ---> Done ({} bytes)", bytes.len());

        info!("--- Unpacking");
        archive::unpack(bytes.as_slice(), &".")?;
        info!("--- ---> Done");

        Ok(true)
    } else {
        info!("The cache doesn't exist on S3.");
        Ok(false)
    }
}

pub fn execute_command(command: &str, verbose: bool) -> Result<(), Error> {
    info!("Execute `{}`", command);

    let stdout = if verbose {
        Stdio::inherit()
    } else {
        Stdio::null()
    };

    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(stdout)
        .output()?;

    Ok(())
}

pub fn upload_archive(s3_context: &S3Context, hex: &str, archive: &str) -> Result<(), Error> {
    let tarball = {
        info!("--- Archiving");
        let bytes = archive::pack_dir(&archive, Vec::new())?;
        info!("--- ---> Done ({} bytes)", bytes.len());

        info!("--- Encoding");
        let bytes = archive::encode(bytes.as_slice(), Vec::new())?;
        info!("--- ---> Done ({} bytes)", bytes.len());

        bytes
    };

    info!("--- Uploading");
    s3::upload(&s3_context, &hex, tarball.to_vec())?;
    info!("--- ---> Done!");

    Ok(())
}
