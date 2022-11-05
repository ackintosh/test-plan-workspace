FROM rust:1.64-bullseye as builder
WORKDIR /usr/src/test-plan

COPY . .
RUN cd ./plan/ && cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/test-plan/plan/target/release/plan_a /usr/local/bin/plan_a

ENTRYPOINT ["plan_a"]