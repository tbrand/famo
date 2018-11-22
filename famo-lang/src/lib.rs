#[macro_use]
extern crate getset;

use std::path::{Path, PathBuf};

#[derive(Clone, Getters)]
pub struct Lang {
    #[get = "pub"]
    name: &'static str,
    #[get = "pub"]
    watches: Vec<&'static str>,
    #[get = "pub"]
    build: &'static str,
    #[get = "pub"]
    command: &'static str,
}

impl Lang {
    fn new(
        name: &'static str,
        watches: Vec<&'static str>,
        build: &'static str,
        command: &'static str,
    ) -> Lang {
        Lang {
            name,
            watches,
            build,
            command,
        }
    }

    fn is_it(&self, path: &AsRef<Path>) -> bool {
        for f in self.watches.iter() {
            let mut path_buf = PathBuf::new();
            path_buf.push(path);
            path_buf.push(f);

            if !path_buf.is_file() {
                return false;
            }
        }

        true
    }
}

pub fn langs() -> Vec<Lang> {
    vec![
        Lang::new(
            "rust",
            vec!["Cargo.toml", "Cargo.lock"],
            "target",
            "cargo build",
        ),
        Lang::new(
            "yarn",
            vec!["package.json", "yarn.lock"],
            "node_modules",
            "yarn build",
        ),
        Lang::new(
            "node_js",
            vec!["package.json", "package-lock.json"],
            "node_modules",
            "npm build",
        ),
        Lang::new(
            "ruby",
            vec!["Gemfile", "Gemfile.lock"],
            "vendor",
            "bundle install --path vendor/bundle",
        ),
        Lang::new(
            "crystal",
            vec!["shard.yaml", "shard.lock"],
            "lib",
            "shards build",
        ),
    ]
}

pub fn detect(path: &AsRef<Path>) -> Option<Lang> {
    for l in langs().iter() {
        if l.is_it(path) {
            return Some(l.clone());
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_rust() {
        let path = Path::new("fixtures/rust");
        let lang = detect(&path).unwrap();

        assert_eq!(lang.name(), &"rust");
        assert_eq!(lang.watches(), &vec!["Cargo.toml", "Cargo.lock"]);
        assert_eq!(lang.build(), &"target");
        assert_eq!(lang.command(), &"cargo build");
    }

    #[test]
    fn detect_node_js() {
        let path = Path::new("fixtures/node_js");
        let lang = detect(&path).unwrap();

        assert_eq!(lang.name(), &"node_js");
        assert_eq!(lang.watches(), &vec!["package.json", "package-lock.json"]);
        assert_eq!(lang.build(), &"node_modules");
        assert_eq!(lang.command(), &"npm build");
    }

    #[test]
    fn detect_yarn() {
        let path = Path::new("fixtures/yarn");
        let lang = detect(&path).unwrap();

        assert_eq!(lang.name(), &"yarn");
        assert_eq!(lang.watches(), &vec!["package.json", "yarn.lock"]);
        assert_eq!(lang.build(), &"node_modules");
        assert_eq!(lang.command(), &"yarn build");
    }

    #[test]
    fn detect_ruby() {
        let path = Path::new("fixtures/ruby");
        let lang = detect(&path).unwrap();

        assert_eq!(lang.name(), &"ruby");
        assert_eq!(lang.watches(), &vec!["Gemfile", "Gemfile.lock"]);
        assert_eq!(lang.build(), &"vendor");
        assert_eq!(lang.command(), &"bundle install --path vendor/bundle");
    }

    #[test]
    fn detect_none() {
        let path = Path::new("fixtures/none");
        let lang = detect(&path);

        assert!(lang.is_none());
    }
}
