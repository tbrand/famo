#[macro_use]
extern crate failure;
#[macro_use]
extern crate getset;
extern crate base64;
extern crate clap;
extern crate reqwest;
extern crate time;
#[macro_use]
extern crate log;
extern crate crypto;

pub mod context;
mod signature;

use context::Context;
use failure::Error;
use reqwest::Response;
use std::time::Duration;

#[derive(Debug, Fail)]
enum S3Error {
    #[fail(display = "Error response from S3 ({})", reason)]
    General { reason: String },
}

fn request() -> Result<reqwest::Client, Error> {
    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(300))
        .build()?;

    Ok(client)
}

pub fn put(context: &Context, key: &str, data: Vec<u8>) -> Result<Response, Error> {
    let url = format!("https://{}/{}/{}", context.endpoint(), context.bucket(), key);

    debug!("Put Object {}", url);

    let date = time::now_utc().rfc822().to_string();
    let authorization = signature::authorization(
        "PUT",
        &date,
        key,
        "",
        "",
        "application/octet-stream",
        context,
    );

    request()?
        .put(&url)
        .header("Date", date)
        .header("Content-Type", "application/octet-stream")
        .header("Content-Length", data.len() as u64)
        .header("Authorization", authorization)
        .body(data)
        .send()
        .map_err(Into::into)
}

pub fn get(context: &Context, key: &str) -> Result<Response, Error> {
    let url = format!("https://{}/{}/{}", context.endpoint(), context.bucket(), key);

    debug!("Get Object {}", url);

    let date = time::now_utc().rfc822().to_string();
    let authorization = signature::authorization("GET", &date, key, "", "", "", context);

    request()?
        .get(&url)
        .header("Date", date)
        .header("Authorization", authorization)
        .send()
        .map_err(Into::into)
}

pub fn get_acl(context: &Context, key: &str) -> Result<Response, Error> {
    let url = format!("https://{}/{}/{}?acl", context.endpoint(), context.bucket(), key);

    debug!("Get Object ACL {}", url);

    let date = time::now_utc().rfc822().to_string();
    let authorization = signature::authorization(
        "GET",
        &date,
        &(key.to_owned() + "?acl"),
        "",
        "",
        "",
        context,
    );

    request()?
        .get(&url)
        .header("Date", date)
        .header("Authorization", authorization)
        .send()
        .map_err(Into::into)
}

pub fn download(context: &Context, key: &str) -> Result<Vec<u8>, Error> {
    let mut response = get(context, key)?;
    let mut data: Vec<u8> = Vec::new();

    if !response.status().is_success() {
        return Err(S3Error::General {
            reason: format!(
                "Get Object: {} ({})",
                response.text()?,
                response.status().as_u16()
            ),
        }.into());
    }

    response.copy_to(&mut data)?;

    Ok(data)
}

pub fn upload(context: &Context, key: &str, data: Vec<u8>) -> Result<(), Error> {
    let mut response = put(context, key, data)?;

    if !response.status().is_success() {
        return Err(S3Error::General {
            reason: format!(
                "Put Object: {} ({})",
                response.text()?,
                response.status().as_u16()
            ),
        }.into());
    }

    Ok(())
}

pub fn key_exists(context: &Context, key: &str) -> Result<bool, Error> {
    let mut response = get_acl(context, key)?;

    match response.status().as_u16() {
        200 => Ok(true),
        404 => Ok(false),
        code => Err(S3Error::General {
            reason: format!("Get Object ACL: {} ({})", response.text()?, code),
        }.into()),
    }
}
