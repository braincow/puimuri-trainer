FROM scratch

ARG PORT=8080
ENV ROCKET_PORT=$PORT
ENV ROCKET_ADDRESS=0.0.0.0

COPY target/x86_64-unknown-linux-musl/release/puimuri-rest /puimuri-rest
COPY static /static

ENTRYPOINT [ "/puimuri-rest" ]
