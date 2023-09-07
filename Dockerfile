FROM lukemathwalker/cargo-chef:latest AS chef

WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
  DEBIAN_FRONTEND=noninteractive \
  apt-get install lld clang -y

FROM chef as planner
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef as build

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Break dependance on online database
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

# Use base instead of static for libc
FROM gcr.io/distroless/base-debian12

WORKDIR /app

# libc needed for binary, just assume the one we built against is the best option.
COPY --from=build /usr/lib/x86_64-linux-gnu/libgcc_s.so.1 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=build /app/configuration/base.yml /app/configuration/
COPY --from=build /app/configuration/production.yml /app/configuration/
COPY --from=build /app/target/release/zero2prod /app/zero2prod

ENV APP_ENVIRONMENT production

CMD ["/app/zero2prod"]

