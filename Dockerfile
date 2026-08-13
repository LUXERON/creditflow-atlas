FROM node:24-bookworm AS web
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM rust:1.96-bookworm AS rustbuild
WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock* ./backend/
COPY backend/src ./backend/src
COPY --from=web /app/frontend/dist ./frontend/dist
WORKDIR /app/backend
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app/backend
COPY --from=rustbuild /app/backend/target/release/creditflow-atlas /usr/local/bin/creditflow-atlas
COPY --from=web /app/frontend/dist /app/frontend/dist
ENV STATIC_DIR=/app/frontend/dist DATABASE_PATH=/app/data/creditflow.db PORT=10000
EXPOSE 10000
CMD ["creditflow-atlas"]
