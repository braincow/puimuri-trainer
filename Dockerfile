FROM scratch

ARG PORT=8000
ENV ROCKET_PORT=$PORT

COPY target/release/puimuri-rest /puimuri-rest
COPY static /static

ENTRYPOINT [ "/puimuri-rest" ]