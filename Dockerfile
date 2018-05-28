FROM alpine:latest

MAINTAINER Thomas Pedoussaut <thomas@ptsynergi.com>

ADD target/release/rusty-sum /usr/local/bin/rusty-sum


CMD ["/usr/local/bin/rusty-sum","2 2"]
