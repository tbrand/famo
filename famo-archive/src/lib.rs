extern crate failure;
extern crate flate2;
extern crate tar;

use failure::Error;
use flate2::write::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::io::prelude::*;
use std::path::Path;
use tar::Archive;

pub fn pack_dir<W>(src_path: &AsRef<Path>, w: W) -> Result<W, Error>
where
    W: Write,
{
    let mut builder = tar::Builder::new(w);

    // https://github.com/alexcrichton/tar-rs/issues/174
    builder.follow_symlinks(false);
    builder.append_dir_all(src_path, src_path)?;
    builder.finish()?;
    Ok(builder.into_inner()?)
}

pub fn pack_file<W>(src_path: &AsRef<Path>, w: W) -> Result<W, Error>
where
    W: Write,
{
    let mut builder = tar::Builder::new(w);

    builder.append_path(src_path)?;
    builder.finish()?;

    Ok(builder.into_inner()?)
}

pub fn unpack<R>(r: R, dist: &AsRef<Path>) -> Result<(), Error>
where
    R: Read,
{
    let mut archive = Archive::new(r);
    archive.unpack(dist)?;

    Ok(())
}

pub fn encode<W>(data: &[u8], w: W) -> Result<W, Error>
where
    W: Write,
{
    let mut encoder = GzEncoder::new(w, Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

pub fn decode<W>(data: &[u8], w: W) -> Result<W, Error>
where
    W: Write,
{
    let mut decoder = GzDecoder::new(w);
    decoder.write_all(data)?;
    Ok(decoder.finish()?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_and_decode() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let encoded = encode(&data, Vec::new()).unwrap();
        let decoded = decode(encoded.as_slice(), Vec::new()).unwrap();

        assert_eq!(data, decoded.as_slice());
    }
}
