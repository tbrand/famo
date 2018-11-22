use clap::ArgMatches;

#[derive(Default, Debug, Getters)]
pub struct Context {
    #[get = "pub"]
    region: String,
    #[get = "pub"]
    endpoint: String,
    #[get = "pub"]
    access_key_id: String,
    #[get = "pub"]
    secret_access_key: String,
    #[get = "pub"]
    bucket: String,
    #[get = "pub"]
    key: Option<String>,
}

impl Context {
    pub fn new(
        region: &str,
        endpoint: &str,
        access_key_id: &str,
        secret_access_key: &str,
        bucket: &str,
        key: Option<&str>,
    ) -> Self {
        Self {
            region: region.to_owned(),
            endpoint: endpoint.to_owned(),
            access_key_id: access_key_id.to_owned(),
            secret_access_key: secret_access_key.to_owned(),
            bucket: bucket.to_owned(),
            key: key.map(|k| k.to_owned()),
        }
    }

    pub fn from_matches(matches: &ArgMatches) -> Self {
        let region = matches.value_of("region").unwrap();
        let endpoint = matches.value_of("endpoint").unwrap();
        let access_key_id = matches.value_of("access_key_id").unwrap();
        let secret_access_key = matches.value_of("secret_access_key").unwrap();
        let bucket = matches.value_of("bucket").unwrap();
        let key = matches.value_of("key");

        Self::new(
            region,
            endpoint,
            access_key_id,
            secret_access_key,
            bucket,
            key,
        )
    }
}
