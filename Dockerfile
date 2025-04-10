# 🔵 ÉTAPE 1 : Utiliser une image officielle Rust pour la compilation
FROM rust:latest as builder

# 🛠️ Définir le répertoire de travail dans le conteneur
WORKDIR /app

# 📦 Copier les fichiers nécessaires pour la compilation
COPY Cargo.toml .
COPY src ./src

# 🔧 Compiler en mode release (optimisé)
RUN cargo build --release

# 🐧 ÉTAPE 2 : Utiliser une image légère pour l'exécution
FROM debian:buster-slim

# 🔄 Copier le binaire compilé depuis l'étape "builder"
COPY --from=builder /app/target/release/rust_doker /usr/local/bin/

# 🚀 Lancer l'application au démarrage du conteneur
CMD ["rust_doker"]