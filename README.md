# Famo

Famo is a command line tool for caching builds on S3 similer to [sccache](https://github.com/mozilla/sccache).
Famo is assumed to be used in many CI/CD platforms and for many languages.

<i>Still under super development.</i>

## Main Features
- :heavy_check_mark: Quite easy and simple to use.
- :heavy_check_mark: Detect languages automatically.
- :heavy_check_mark: Single Binary (For Linux, MacOS) which could be easily installed on CI/CD platforms.
- :heavy_check_mark: Asyncronous execution. (TBD)

## Quick Start

To use famo, you need
- :heavy_check_mark: `famo` command
- :heavy_check_mark: S3 or its compatibles with access key and secret access key.

## Install famo command
TBD

## Setting up S3
TBD

```bash
export FAMO_ACCESS_KEY=
export FAMO_SECRET_ACCESS_LEY=
```

## Use it!
TBD

# Options
TBD

# How does Famo work?

1. Watch package files (like Cargo.toml, Gemfile, package.json ...).
1. Calculate unique hash (hex) from them.
1. Check the existance of the cache on S3.
1. If cache exists, download and unpack it into current directory.
1. Builds project. If the cache was hit, it's very fast.
1. Upload an archive of the cache if cache didn't exist on step 4.

So basically cache is effective until package files are not changed.
The cache will reduce the build time especially for the big projects.

# Comparing to similer works

- [sccache](https://github.com/mozilla/sccache)
- [ccache](https://ccache.samba.org/)

TBD

# Supports

## Supported Languages (Package Managers)

- Rust (cargo)
- Node.js (npm, yarn)
- Ruby (gem)
- Crystal (shards)

## Supported Platforms

Basically, `famo` can be used in every platforms including CircleCI, TravisCI and so on.
Here are basic steps to use famo in your platform.

- Pre-download `famo` or include `famo` in build image.
- Setting up S3 environment on your build platform.
- Insert famo step into your build process.

## Supported Backend

Currently only S3 and its compatibles are supported.
Please suggest on issues when you need others.

# Development and Contribution
