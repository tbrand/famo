use context::Context;
use ring::digest;
use ring::hmac::{self, SigningKey};

pub fn authorization(
    verb: &str,
    date: &str,
    key: &str,
    md5: &str,
    headers: &str,
    content_type: &str,
    context: &Context,
) -> String {
    let string = format!(
        "{verb}\n{md5}\n{content_type}\n{date}\n{headers}{resource}",
        verb = verb,
        md5 = md5,
        content_type = content_type,
        date = date,
        headers = headers,
        resource = format!("/{}/{}", context.bucket(), key)
    );

    let signature = sign(string.as_bytes(), context.secret_access_key());
    format!("AWS {}:{}", context.access_key_id(), signature)
}

fn sign(data: &[u8], secret_access_key: &str) -> String {
    let s = hmac(secret_access_key, data);
    base64::encode_config::<Vec<u8>>(&s, base64::STANDARD)
}

fn hmac(key: &str, data: &[u8]) -> Vec<u8> {
    let key = SigningKey::new(&digest::SHA1, key.to_owned().as_bytes());
    hmac::sign(&key, data).as_ref().to_vec()
}
