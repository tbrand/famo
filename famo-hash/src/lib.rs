extern crate failure;
extern crate num;
extern crate sha2;

use failure::Error;
use num::bigint::BigUint;
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};

///
/// Calculate unique hex from paths of files and directories.
///
/// ```rust
/// use famo_hash::hex;
///
/// let paths = vec!["Cargo.toml", "Cargo.lock"];
/// let hex = hex(&paths).unwrap();
/// println!("The unique hex is {}", hex);
/// ```
pub fn hex(paths: &Vec<&str>) -> Result<String, Error> {
    let s = sum(paths)?;
    let h = format!("{:x}", s);

    Ok(h)
}

///
/// Calculate sum of each BigUint of files.
///
/// ```rust
/// use famo_hash::sum;
///
/// let paths = vec!["Cargo.toml", "Cargo.lock", "target"];
/// let sum = sum(&paths).unwrap();
/// println!("The sum of these files is {}", sum);
/// ```
pub fn sum(paths: &Vec<&str>) -> Result<BigUint, Error> {
    let s = read(paths)?.iter().sum::<BigUint>();

    Ok(s)
}

///
/// Create a set of BigUint of each files.
/// They are calculated from its contents and path.
///
/// ```rust
/// use famo_hash::read;
///
/// let paths = vec!["Cargo.toml", "Cargo.lock", "target"];
/// let vals = read(&paths).unwrap();
/// ```
///
pub fn read(paths: &Vec<&str>) -> Result<Vec<BigUint>, Error> {
    let mut v: Vec<BigUint> = vec![];

    for path in paths.iter() {
        let path = Path::new(path);

        if path.is_file() {
            read_file(path, &mut v)?;
        } else if path.is_dir() {
            read_dir(path, &mut v)?;
        }
    }

    Ok(v)
}

///
/// Recursively read directory and create a set of BigUint of each files.
/// The result is pushed into Vec<BigUint>.
///
/// ```rust
/// extern crate num; // Use external crate 'num'
/// extern crate famo_hash;
///
/// // If you use extern famo_lib, this line should be
/// // `use famo_lib::hash::read_dir;`
/// use famo_hash::read_dir;
/// use num::bigint::BigUint;
/// use std::path::Path;
///
/// let mut v: Vec<BigUint> = vec![];
/// let path = Path::new("src");
/// read_dir(&path, &mut v).unwrap();
/// ```
///
pub fn read_dir(path: &Path, v: &mut Vec<BigUint>) -> Result<(), Error> {
    for file_or_dir in path.read_dir()? {
        if let Ok(file_or_dir) = file_or_dir {
            let file_or_dir = file_or_dir.path();

            if file_or_dir.is_file() {
                read_file(file_or_dir.as_path(), v)?;
            } else if file_or_dir.is_dir() {
                read_dir(file_or_dir.as_path(), v)?;
            }
        }
    }

    Ok(())
}

///
/// Read a single file to create a unique BigUint.
/// The value is created from its contents and path.
/// The result is pushed into Vec<BigUint>.
///
/// ```rust
/// extern crate num; // Use external crate 'num'
/// extern crate famo_hash;
///
/// // If you use extern famo_lib, this line should be
/// // `use famo_lib::hash::read_file;`
/// use famo_hash::read_file;
/// use num::bigint::BigUint;
/// use std::path::Path;
///
/// let mut v: Vec<BigUint> = vec![];
/// let path = Path::new("Cargo.toml");
/// read_file(&path, &mut v).unwrap();
/// ```
///
pub fn read_file(path: &Path, v: &mut Vec<BigUint>) -> Result<(), Error> {
    let c = unique_contents(path)?;
    let n = gen_biguint(c.as_slice());

    v.push(n);

    Ok(())
}

// Path -> Vec<u8>
fn unique_contents(path: &Path) -> Result<Vec<u8>, Error> {
    let mut contents = fs::read(path)?;
    contents.append(&mut path.to_str().unwrap().as_bytes().to_vec());

    Ok(contents)
}

// &[u8] -> SHA256 -> BigUint
fn gen_biguint(bytes: &[u8]) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.input(bytes);
    let bytes: &[u8] = &hasher.result();
    let n = BigUint::from_bytes_be(bytes);

    n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gen_biguint_success() {
        let string = "famo".to_owned();
        let n = gen_biguint(string.as_bytes());

        assert_eq!(
            "70685181624748995639890562460754138405737812120862994921776424960744339326042"
                .to_owned(),
            format!("{}", n)
        );
    }

    #[test]
    fn unique_contents_of_single_file() {
        let bytes = vec![
            48, 102, 105, 120, 116, 117, 114, 101, 115, 47,
            117, 110, 105, 113, 117, 101, 95, 99, 111, 110,
            116, 101, 110, 116, 115, 95, 111, 102, 95, 115,
            105, 110, 103, 108, 101, 95, 102, 105, 108, 101,
            47, 102, 105, 108, 101
        ];

        let path = Path::new("fixtures/unique_contents_of_single_file/file");
        let v = unique_contents(&path).unwrap();

        assert_eq!(v, bytes);
    }

    #[test]
    fn unique_contents_are_not_same_if_paths_are_different() {
        let path0 = Path::new("fixtures/unique_contents_are_not_same_if_paths_are_different/file0");
        let path1 = Path::new("fixtures/unique_contents_are_not_same_if_paths_are_different/file1");

        let v0 = unique_contents(&path0).unwrap();
        let v1 = unique_contents(&path1).unwrap();

        assert_ne!(v0, v1);
    }
}
