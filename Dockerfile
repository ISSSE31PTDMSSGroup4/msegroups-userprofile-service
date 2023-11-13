# FROM rust:1.71-slim-buster
FROM rustlang/rust:nightly-buster-slim  AS builder

# Environment variable for rocket
ENV ROCKET_PROFILE=production

WORKDIR /app

COPY userprofile-api .
COPY .env .
RUN cargo build --release

FROM debian:buster-slim as runner
WORKDIR /app
RUN mkdir -p /app/uploads
RUN apt update
RUN apt install ca-certificates -y

COPY --from=builder /app/target/release/userprofile-api /app/userprofile-api
# COPY .env ../
COPY userprofile-api/Rocket.toml .

# Environment variable for rocket
ENV ROCKET_PROFILE=production

EXPOSE 8001

# ENTRYPOINT ["cargo"]
# CMD ["run"]
CMD ["/app/userprofile-api", "--config=/app/"]
