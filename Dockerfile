FROM node:alpine as node_build
WORKDIR /workdir
COPY . .
WORKDIR /workdir/frontend
RUN npm install
RUN npm run build

FROM rust:alpine AS rust_build
WORKDIR /workdir
COPY . .
RUN apk add --no-cache musl musl-dev
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
ARG PORT=8000
ARG ADDRESS=0.0.0.0
ENV ROCKET_PORT=$PORT
ENV ROCKET_ADDRESS=$ADDRESS
COPY --from=rust_build /workdir/target/x86_64-unknown-linux-musl/release/puimuri-trainer /
COPY --from=node_build /workdir/frontend/dist/* /static
EXPOSE ${ROCKET_PORT}
ENTRYPOINT [ "/puimuri-trainer" ]
