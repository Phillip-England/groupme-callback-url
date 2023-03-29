# FROM rust:1.67

# WORKDIR /usr/src/app

# COPY . .

# CMD ["cargo", "run"]

# EXPOSE 8000


FROM rust:1.67

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

EXPOSE 8000

CMD ["rust-api"]
