FROM amazonlinux:2018.03

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN yum install gcc openssl-devel pkg-config -y

WORKDIR /usr/src/assemblylift
COPY . .

RUN $HOME/.cargo/bin/cargo build --release
RUN cp /usr/src/assemblylift/target/release/bootstrap /github/workspace
