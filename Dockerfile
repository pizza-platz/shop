FROM debian:buster

COPY ./target/release/pizza-platz-shop /usr/local/bin/

CMD ["/usr/local/bin/pizza-platz-shop"]
