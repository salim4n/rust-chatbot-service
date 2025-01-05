# Build stage
FROM rust:latest as builder

# Créer le répertoire de travail
WORKDIR /usr/src/app

# Copier tous les fichiers source
COPY . .

# Nettoyer tout build précédent
RUN cargo clean

# Build en mode release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Installer les dépendances nécessaires
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copier l'exécutable depuis le builder
COPY --from=builder /usr/src/app/target/release/chatbot-service /usr/local/bin/chatbot-service

# Définir l'exécutable comme point d'entrée
ENTRYPOINT ["chatbot-service"]