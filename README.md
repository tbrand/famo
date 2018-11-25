# Famo

Famo is a command line tool for caching builds on S3 similer to [sccache](https://github.com/mozilla/sccache).
Famo is assumed to be used in many CI/CD platforms and for many languages.

<i>Still under super development.</i>

## Main Features
- :heavy_check_mark: Quite easy and simple to use.
- :heavy_check_mark: Detect languages automatically.
- :heavy_check_mark: [WIP] Single Binary (For Linux, MacOS) which could be easily installed on CI/CD platforms.
- :heavy_check_mark: [WIP] Asyncronous uploading.

## Quick Start

To use famo, you need
- :heavy_check_mark: `famo` command
- :heavy_check_mark: S3 or its compatibles with access key and secret access key.

### Install famo command
TBD

### Setting up S3
You need below information about S3.
- Access Key ID (e.g. AKIAXXXXXXXXXXXXXXXX)
- Secret Access Key (e.g. HLn2U3xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx)
- Region (e.g. ap-northeast-1)
- Endpoint (e.g. s3-ap-northeast-1.amazonaws.com)
- Bucket (e.g. famo-cache)

You could pass them as command line arguments.
But access key id and secret access key should be passed as environment variables to hide their actual values.
Use systems called "secrets" or "credentials" on your CI/CD platforms to set them.
```bash
export FAMO_ACCESS_KEY=AKIAXXXXXXXXXXXXXXXX
export FAMO_SECRET_ACCESS_LEY=HLn2U3xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### Use it!
```bash
famo --bucket=famo-cache --region=ap-northeast-1 --endpoint=s3-ap-northeast-1.amazonaws.com
```

### Options
`famo -h` to show other options.

## How does Famo work?
1. Watch package files (like Cargo.toml, Gemfile, package.json ...).
1. Calculate unique hash (hex) from them.
1. Check the existance of the cache on S3.
1. If the cache exists, download and unpack it into current directory.
1. Builds project. If the cache was hit, it's very fast.
1. Upload an archive of the cache if cache didn't exist on step 4.

So basically cache is effective until package files are not changed.
The cache will reduce the build time especially for the big projects.

## Comparing to similer works
There are several works which Famo was inspired. `sscache` developed by Mozilla is also caching builds onto cloud storages but supported languages (build tools) are limited.
Also, it's assumed to be used in local build, not on CI/CD. You could use it on it by pre-install it into your build images but it's not good from the aspect of devops.
`sccache` is inspired by ccache which only support `gcc`.
- [sccache](https://github.com/mozilla/sccache)
- [ccache](https://ccache.samba.org/)

CircleCI, TravisCI and other famous platforms have their caching system.
Famo is used in platforms which doesn't have such system.
- [CircleCI's caching system](https://circleci.com/docs/2.0/caching/)
- [TravisCI's caching system](https://docs.travis-ci.com/user/caching/)

## Supports

### Supported Languages (Package Managers)
They are targets of auto detection. You can use Famo to other languages (not be listed below) by specifying `--watches` and `--archive` options.
- Rust (cargo)
- Node.js (npm, yarn)
- Ruby (gem)
- Crystal (shards)

### Supported Platforms
Basically, Famo can be used in every platforms.
Here are basic steps to use famo in your platform.

- Pre-download `famo` or include `famo` in build image.
- Setting up S3 environment on your build platform.
- Insert famo step into your build process.

### Supported Backend
Currently only S3 and its compatibles are supported.
Please suggest on issues when you need others. (e.g. Redis)

## Development and Contribution
We need your supports especially for
- Adding another languages as a target of auto detection.
- Adding another awesome option. (e.g. Asyncronous uploading)
- Bug fixes

We don't need issues for each pull request. Please submit pull request directly. :smile:
