FROM rust:1.67 as builder

WORKDIR /usr/src/
COPY . .


RUN cargo check
RUN make build


FROM debian:bullseye-slim

COPY --from=builder /usr/src/target/release/robot /usr/local/bin/robot
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

#RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /etc/robot/

#ADD ./Rocket.toml /etc/robot

CMD ["robot"]