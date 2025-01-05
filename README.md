# Chatbot Service IgnitionAI

Ce projet est un service de chatbot intelligent développé pour IgnitionAI, une agence spécialisée en intelligence artificielle. Il utilise des techniques avancées de traitement du langage naturel et de recherche sémantique pour fournir des réponses pertinentes aux utilisateurs.

## Fonctionnalités

- Chatbot conversationnel basé sur le modèle GPT-4
- Recherche sémantique utilisant des embeddings de texte
- Stockage et récupération de vecteurs d'information via Azure Table Storage
- API RESTful pour l'interaction avec le chatbot

## Prérequis

- Rust (édition 2021 ou supérieure)
- Compte Azure avec accès à Azure Table Storage
- Clé API OpenAI

## Configuration

1. Clonez le dépôt
2. Créez un fichier `.env` à la racine du projet avec les variables suivantes :
   ```
   STORAGE_ACCOUNT=votre_compte_de_stockage_azure
   STORAGE_ACCESS_KEY=votre_clé_d_accès_azure
   STORAGE_TABLE_NAME=nom_de_votre_table_azure
   OPENAI_API_KEY=votre_clé_api_openai
   ```

## Installation

```bash
cargo build --release
```

## Utilisation

Pour démarrer le serveur :

```bash
cargo run --release
```

Le serveur démarrera sur `http://0.0.0.0:3000`.

## Endpoints API

- `GET /` : Page d'accueil
- `POST /chat` : Envoyer un message au chatbot
- `GET /vectors` : Récupérer tous les vecteurs stockés

## Structure du projet

- `src/main.rs` : Point d'entrée de l'application et configuration du serveur
- `src/agent.rs` : Logique du chatbot et traitement des requêtes
- `src/azure_table.rs` : Interaction avec Azure Table Storage

## Contribution

Les contributions sont les bienvenues. Veuillez ouvrir une issue pour discuter des modifications majeures que vous souhaitez apporter.
