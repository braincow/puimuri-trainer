FROM scratch

ARG PORT=8000
ARG ADDRESS=0.0.0.0

ENV ROCKET_PORT=$PORT
ENV ROCKET_ADDRESS=$ADDRESS

COPY target/x86_64-unknown-linux-musl/release/puimuri-trainer /
COPY static /static

ENTRYPOINT [ "/puimuri-trainer" ]
