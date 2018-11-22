FROM ekidd/rust-musl-builder

WORKDIR /home/rust

COPY Cargo.toml Cargo.lock ./
COPY famo-archive famo-archive
COPY famo-hash famo-hash
COPY famo-lang famo-lang
COPY famo-s3 famo-s3
COPY famo-lib famo-lib
COPY src src

RUN cargo build --release
