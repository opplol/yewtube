FROM rust:1.66-buster

RUN apt-get update && \
    apt-get -y install git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rust-analyzer rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch && \
    rustup target add wasm32-unknown-unknown && \
    cargo install --locked trunk

RUN curl -sL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt install -y nodejs && \
    npm i -g tailwindcss


