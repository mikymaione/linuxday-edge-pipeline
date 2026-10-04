# Linux Day Edge Pipeline

Un'architettura di elaborazione dati *edge* e *microservices-in-Wasm* basata su **Spin v4.2.1** e **Rust**, progettata per essere eseguita in locale su **Linux** o distribuita su **Fermyon Cloud**.

## 📐 Architettura

Il progetto è organizzato come una **Cargo Workspace** composta da tre componenti WebAssembly indipendenti che comunicano tra loro tramite chiamate HTTP/Wasm:


```
              ┌─────────────────┐
              │   HTTP Client   │
              └────────┬────────┘
                       │ POST /api/...
                       ▼
              ┌───────────────────┐
              │  edge-gateway     │
              └────┬─────────┬────┘
                   │         │
      POST /enrich │         │ POST /audit
                   ▼         ▼
┌────────────────────────┐ ┌────────────────────────┐
│     data-enricher      │ │      audit-logger      │
│  (Arricchimento JSON)  │ │  (Spin Native SQLite)  │
└────────────────────────┘ └────────────────────────┘

```

1. **`edge-gateway`**: L'entrypoint dell'applicazione (`/api/...`). Riceve le richieste esterne, le inoltra al servizio di enrichment, invia l'esito all'audit logger e restituisce la risposta finale.
2. **`data-enricher`**: Microservizio HTTP (`/enrich`) che riceve il payload JSON originale e vi aggiunge metadati di tracciamento.
3. **`audit-logger`**: Microservizio HTTP (`/audit`) che persiste i payload ricevuti su un database SQLite nativo gestito direttamente dal runtime Spin.

---

## 🌐 Live Demo & Cloud Deployment

L'applicazione è attualmente pubblicata su **Fermyon Cloud**:

* **Dashboard di gestione**: [linuxday-edge-pipeline su Fermyon Cloud](https://cloud.fermyon.com/app/linuxday-edge-pipeline/activity)
* **Endpoint pubblico (API Gateway)**: `https://linuxday-edge-pipeline.fermyon.app/api/...`

### Test End-to-End dell'endpoint Cloud

```bash
curl -i -X POST https://linuxday-edge-pipeline.fermyon.app/api/test \
  -H "Content-Type: application/json" \
  -d '{"event": "Linux Day 2026", "status": "active"}'

```

---

## 🛠️ Requisiti di Sistema (Debian)

Tutti i prerequisiti si installano tramite APT nativo:

```bash
sudo apt update
sudo apt install -y build-essential git pkg-config libssl-dev rustc cargo libstd-rust-dev-wasm32

```

Richiede inoltre **Spin CLI v4.2.1** installato nel sistema (`/usr/local/bin/spin`) con il plugin `cloud`:

```bash
spin plugin install cloud

```

---

## 🚀 Struttura del Progetto

```text
linuxday-edge-pipeline/
├── Cargo.toml               # Configurazione Cargo Workspace
├── spin.toml                # Manifest delle componenti e route Spin
├── .gitignore
├── LICENSE                  # Testo completo della Licenza GNU GPLv3
├── README.md
├── edge-gateway/            # Componente Gateway API
│   ├── Cargo.toml
│   └── src/lib.rs
├── data-enricher/           # Componente Data Enricher
│   ├── Cargo.toml
│   └── src/lib.rs
└── audit-logger/            # Componente Audit Logger (SQLite)
    ├── Cargo.toml
    └── src/lib.rs

```

---

## 🔨 Compilazione

Dalla radice del progetto, esegui il comando di build di Spin che invocherà `cargo build --target wasm32-wasip1 --release` per ciascun componente della workspace:

```bash
spin build

```

Gli artefatti `.wasm` verranno generati all'interno della cartella `target/wasm32-wasip1/release/`.

---

## 🏃 Esecuzione Locale e Deploy

### Avvio locale

```bash
spin up

```

L'applicazione sarà in ascolto su `http://127.0.0.1:3000`.

### Deploy su Fermyon Cloud

```bash
spin cloud login
spin cloud deploy

```

---

## 🧪 Testing Locale

Invia una richiesta di prova al Gateway locale:

```bash
curl -i -X POST [http://127.0.0.1:3000/api/test](http://127.0.0.1:3000/api/test) \
  -H "Content-Type: application/json" \
  -d '{"event": "Linux Day 2026", "status": "active"}'

```

### Verifica Audit Log (SQLite Locale)

Per verificare che l'evento sia stato registrato nel database SQLite interno di Spin (`.spin/sqlite_db.db`):

```bash
sqlite3 .spin/sqlite_db.db "SELECT * FROM audit_logs;"

```

---

## 📜 Licenza

Questo progetto è software libero ed è distribuito sotto i termini della licenza [GNU General Public License v3.0](https://www.google.com/search?q=LICENSE).

Copyright 2026 (c) [MAIONE MIKY]. All rights reserved.