FROM --platform=$BUILDPLATFORM rust:1.84-slim-bookworm AS builder

ARG TARGETOS TARGETARCH
RUN echo "$TARGETARCH" | sed 's,arm,aarch,;s,amd,x86_,' > /tmp/arch

RUN apt-get update && apt-get install -y "gcc-$(tr _ - < /tmp/arch)-linux-gnu" "g++-$(tr _ - < /tmp/arch)-linux-gnu" make && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN rustup target add "$(cat /tmp/arch)-unknown-${TARGETOS}-gnu"

WORKDIR /usr/src/remote-signer

COPY Cargo.toml Cargo.lock .
COPY examples/ examples
COPY lightspark/ lightspark
COPY lightspark-remote-signing/ lightspark-remote-signing/

RUN cargo build -r --bin lightspark-remote-signing-server 

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates bash htop && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN addgroup --system --gid 1000 sparknode
RUN adduser --system --uid 1000 --home /home/sparknode --ingroup sparknode sparknode

ENTRYPOINT ["remote-signing-server"]
EXPOSE 8000

COPY --from=builder /usr/src/remote-signer/target/release/lightspark-remote-signing-server /usr/local/bin/remote-signing-server

# Install security updates
RUN apt-get update && apt-get -y upgrade && apt-get clean && rm -rf /var/lib/apt/lists

RUN remote-signing-server
