FROM rust:1.78-slim-bookworm as builder

WORKDIR /usr/src/ripfy-server
COPY . .
RUN cargo run --bin migrator
RUN cargo install --path .


FROM debian:bookworm-slim
WORKDIR /usr/src/ripfy-server

RUN apt-get update && apt-get install -y curl ffmpeg && rm -rf /var/lib/apt/lists/*
RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
RUN chmod a+rx /usr/local/bin/yt-dlp

COPY --from=builder /usr/local/cargo/bin/ripfy-server /usr/local/bin/ripfy-server
COPY --from=builder /usr/src/ripfy-server/ripfy.sqlite /usr/src/ripfy-server/ripfy.sqlite

CMD ["ripfy-server"]
