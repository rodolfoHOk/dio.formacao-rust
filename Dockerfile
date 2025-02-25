FROM rust:1.85.0-bookworm

WORKDIR /usr/src/app

CMD [ "tail", "-f", "/dev/null" ]
