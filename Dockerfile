# FROM rust:1.67

# WORKDIR /usr/src/app

# COPY . .

# CMD ["cargo", "run"]

# EXPOSE 8000


FROM rust:1.67

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["myapp"]

EXPOSE 8000