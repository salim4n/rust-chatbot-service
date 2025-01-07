# Chatbot Service IgnitionAI  

**FRANÇAIS**  
Ce projet est un service de chatbot intelligent développé pour IgnitionAI, une agence spécialisée en intelligence artificielle. Il utilise des techniques avancées de traitement du langage naturel et de recherche sémantique pour fournir des réponses pertinentes aux utilisateurs.  

**ENGLISH**  
This project is an intelligent chatbot service developed for IgnitionAI, an agency specializing in artificial intelligence. It uses advanced natural language processing and semantic search techniques to provide relevant responses to users.  

---

## Fonctionnalités / Features  

- **FR** : Chatbot conversationnel basé sur le modèle GPT-4  
- **EN** : Conversational chatbot powered by GPT-4  
- **FR** : Recherche sémantique utilisant des embeddings de texte  
- **EN** : Semantic search using text embeddings  
- **FR** : Stockage et récupération de vecteurs d'information via Azure Table Storage  
- **EN** : Storage and retrieval of information vectors via Azure Table Storage  
- **FR** : API RESTful pour l'interaction avec le chatbot  
- **EN** : RESTful API for chatbot interaction  

---

## Prérequis / Prerequisites  

- **FR** : Rust (édition 2021 ou supérieure)  
- **EN** : Rust (2021 edition or later)  
- **FR** : Compte Azure avec accès à Azure Table Storage  
- **EN** : Azure account with access to Azure Table Storage  
- **FR** : Clé API OpenAI  
- **EN** : OpenAI API key  

---

## Configuration  

**FR** :  
1. Clonez le dépôt  
2. Créez un fichier `.env` à la racine du projet avec les variables suivantes :  
   ```  
   STORAGE_ACCOUNT=votre_compte_de_stockage_azure  
   STORAGE_ACCESS_KEY=votre_clé_d_accès_azure  
   STORAGE_TABLE_NAME=nom_de_votre_table_azure  
   OPENAI_API_KEY=votre_clé_api_openai  
   ```  

**EN**:  
1. Clone the repository  
2. Create a `.env` file at the root of the project with the following variables:  
   ```  
   STORAGE_ACCOUNT=your_azure_storage_account  
   STORAGE_ACCESS_KEY=your_azure_access_key  
   STORAGE_TABLE_NAME=your_azure_table_name  
   OPENAI_API_KEY=your_openai_api_key  
   ```  

---

## Installation  

**FR** :  
```bash  
cargo build --release  
```  

**EN**:  
```bash  
cargo build --release  
```  

---

## Utilisation / Usage  

**FR** : Pour démarrer le serveur :  
**EN**: To start the server:  

```bash  
cargo run --release  
```  

**FR** : Le serveur démarrera sur `http://0.0.0.0:3000`.  
**EN**: The server will start on `http://0.0.0.0:3000`.  

---

## Endpoints API  

- **GET /** :  
  - **FR** : Page d'accueil  
  - **EN** : Homepage  

- **POST /chat** :  
  - **FR** : Envoyer un message au chatbot  
  - **EN** : Send a message to the chatbot  

- **GET /vectors** :  
  - **FR** : Récupérer tous les vecteurs stockés  
  - **EN** : Retrieve all stored vectors  

---

## Structure du projet / Project Structure  

- **FR** : `src/main.rs` : Point d'entrée de l'application et configuration du serveur  
- **EN** : `src/main.rs`: Application entry point and server configuration  

- **FR** : `src/agent.rs` : Logique du chatbot et traitement des requêtes  
- **EN** : `src/agent.rs`: Chatbot logic and request handling  

- **FR** : `src/azure_table.rs` : Interaction avec Azure Table Storage  
- **EN** : `src/azure_table.rs`: Interaction with Azure Table Storage  

---
