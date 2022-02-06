FROM rust

WORKDIR /app
# COPY . .

# RUN cargo build

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

CMD ["cargo", "run", "--bin", "sprd-backend"]