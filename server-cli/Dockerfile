FROM debian:stable-slim
ADD sources-http.list /etc/apt/sources.list
RUN apt update
RUN apt install -y ca-certificates
ADD sources.list /etc/apt/sources.list
# SIGUSR1 causes hyperworld-server-cli to initiate a graceful shutdown
LABEL com.centurylinklabs.watchtower.stop-signal="SIGUSR1"

ARG PROJECTNAME=server-cli

# librust-backtrace+libbacktrace-dev = backtrace functionality
# iproute2 and net-tools for diagnostic purposes
RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends --assume-yes \
        ca-certificates \
        librust-backtrace+libbacktrace-dev \
        iproute2 \
        net-tools \
    && rm -rf /var/lib/apt/lists/*;

COPY ./test.txt /opt/test.txt
COPY hyperworld-server-cli /opt/hyperworld-server-cli
COPY ./assets/common /opt/assets/common
COPY ./assets/server /opt/assets/server
COPY ./assets/world /opt/assets/world

WORKDIR /opt

ENV RUST_BACKTRACE=full
ENTRYPOINT ["/opt/hyperworld-server-cli"]
