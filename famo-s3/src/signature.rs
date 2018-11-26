use context::Context;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

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
    let mut hmac = Hmac::new(Sha1::new(), key.to_owned().as_bytes());
    hmac.input(data);
    hmac.result().code().iter().map(|b| *b).collect::<Vec<u8>>()
}
