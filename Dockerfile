# Temporary builder image to create executable
FROM rust:slim-bookworm AS builder

# Copy over src files
WORKDIR /src
COPY . .

# Add dependencies
# RUN apk add --no-cache libc-dev && \
#     apk add --no-cache postgresql-dev && \
#     apk add --no-cache postgresql-client && \
#     apk add --no-cache postgresql && \
#     apk add --no-cache postgresql-contrib && \
#     apk add --no-cache libpq-dev

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends libpq-dev

# Build executable
RUN cargo build --release

# ------------------------------------------------------------- #

# Minimal base image to run executable
FROM debian:bookworm-slim

# Args for rootless environment
ARG UNAME=actix
ARG UID=1000
ARG GID=1000

ARG PORT=3000

# Install dependencies
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends libpq5

# Setup rootless user
RUN groupadd --gid $GID $UNAME && \
    useradd --no-create-home --home "$(pwd)" --comment "" --gid $UNAME --uid $UID $UNAME

# Copy executable from previous builder image
WORKDIR /usr/src/deadpool-actix-diesel
COPY --from=builder /src/target/release/deadpool-actix-diesel ./

# Run as rootless user
USER $UNAME

EXPOSE $PORT

CMD ["./deadpool-actix-diesel"]
