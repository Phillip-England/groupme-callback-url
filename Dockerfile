FROM rust:1.67

WORKDIR /usr/src/app

COPY . .

CMD ["cargo", "run"]

EXPOSE 8000