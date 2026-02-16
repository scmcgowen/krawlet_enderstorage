FROM rust:slim-bookworm AS build
WORKDIR /usr/src/storage
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /usr/src/storage/target/release/krawlet_enderstorage_api /usr/local/bin/storage
CMD ["/usr/local/bin/storage"]
