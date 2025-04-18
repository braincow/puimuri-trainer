FROM rust:alpine AS rust_build
WORKDIR /workdir
COPY . .
RUN apk add --no-cache musl musl-dev
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM node:alpine as node_build
WORKDIR /workdir
COPY . .
WORKDIR /workdir/frontend
RUN npm install
RUN npm run build

FROM scratch
ENV PORT 8080
ENV ADDRESS 0.0.0.0
COPY --from=rust_build /workdir/target/x86_64-unknown-linux-musl/release/puimuri-trainer /
COPY --from=node_build /workdir/frontend/dist /static
EXPOSE ${PORT}
ENTRYPOINT [ "/puimuri-trainer" ]
