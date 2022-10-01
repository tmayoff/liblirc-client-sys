# base pre-built cross image
FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu

ENV DEBIAN_FRONTEND noninteractive

# add our foreign architecture and install our dependencies
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils
RUN apt-get update && apt-get -y install git xsltproc python3 tclsh 

RUN apt-get -y install liblircclient-dev libsqlite-dev lirc
