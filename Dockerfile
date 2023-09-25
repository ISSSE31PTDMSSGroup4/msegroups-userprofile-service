FROM rust:1.71-slim-buster

# Environment variable for rocket
ENV ROCKET_PROFILE=production

WORKDIR /usr/src/app

COPY userprofile-api .
COPY .env .

RUN cargo build

EXPOSE 8001

ENTRYPOINT ["cargo"]
CMD ["run"]
