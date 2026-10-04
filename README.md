# Linux Day Edge Pipeline

Un'architettura di elaborazione dati *edge* e *microservices-in-Wasm* basata su **Spin v4.2.1** e **Rust**.

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
   POST /enrich    │         │   POST /audit
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

## 🛠️ Requisiti di Sistema (Debian 13)
Richiede **Spin CLI v4.2.1** installato nel sistema (`/usr/local/bin/spin`).

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

## 🏃 Esecuzione

Avvia il runtime Spin:

```bash
spin up

```

L'applicazione sarà in ascolto su `[http://127.0.0.1:3000](http://127.0.0.1:3000)` con le seguenti rotte attive:

* `[http://127.0.0.1:3000/api/](http://127.0.0.1:3000/api/)...` (`edge-gateway`)
* `[http://127.0.0.1:3000/enrich](http://127.0.0.1:3000/enrich)` (`data-enricher`)
* `[http://127.0.0.1:3000/audit](http://127.0.0.1:3000/audit)` (`audit-logger`)

---

## 🧪 Testing End-to-End

Invia una richiesta di prova al Gateway tramite `curl`:

```bash
curl -i -X POST http://127.0.0.1:3000/api/test \
  -H "Content-Type: application/json" \
  -d '{"event": "Linux Day 2026", "status": "active"}'

```

### Verifica Risposta

Dovresti ricevere uno stato `HTTP/1.1 200 OK` con il payload arricchito:

```json
{
  "event": "Linux Day 2026",
  "status": "active",
  "processed_by": "data-enricher-edge"
}

```

### Verifica Audit Log (SQLite)

Per verificare che l'evento sia stato registrato nel database SQLite interno di Spin (`.spin/sqlite_db.db`):

```bash
sqlite3 .spin/sqlite_db.db "SELECT * FROM audit_logs;"

```

---

## 📜 Licenza

Copyright 2026 (c) [MAIONE MIKY]. All rights reserved.

Questo progetto è software libero ed è distribuito sotto i termini della licenza [GNU General Public License v3.0](LICENSE)