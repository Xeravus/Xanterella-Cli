# Xanterella Ecosystem - Entwicklungs-Roadmap

Diese Roadmap basiert auf der geplanten Architektur (Server, Web, Caching, Homelab-Integration) und dem aktuellen Code-Stand. Sie ist chronologisch nach Abhängigkeiten und architektonischen Knotenpunkten sortiert.

## Phase 1: Backend Foundation & API (Server) - *Größtenteils Abgeschlossen*
Der Kern der Datenhaltung und die Schnittstelle zum Dateisystem (Prolyxena).

- [x] **Datenbank-Design (SQLite)**
  - [x] Tabelle `hosts` (inkl. system, remote_deploy, imports, config, etc.)
  - [x] Tabelle `profiles`
  - [x] Tabelle `modules`
  - [x] Tabelle `templates`
  - [x] Tabelle `raw` (BLOB für Binärdaten, readable Flag)
- [x] **Axum REST-API (CRUD-Endpunkte)**
  - [x] `/hosts` (GET, POST, DELETE)
  - [x] `/profiles` (GET, POST, DELETE)
  - [x] `/modules` (GET, POST, DELETE)
  - [x] `/templates` (GET, POST, DELETE)
  - [x] `/raw` (GET, POST, DELETE) - *inklusive sicherem Binär-Upload*
- [x] **VFS & Extraction (Nixtractor)**
  - [x] Nix-AST Parsing aus dem Dateisystem.
  - [x] Relatives Pfad-Handling (`strip_prefix`, Hash-Fallback für kaputtes UTF-8).
- [x] **Live-Event System**
  - [x] Server-Sent Events (SSE) für Status-Updates an das Frontend.

## Phase 2: Web - Config Modifikation Interface 
Das interaktive Dashboard (Arcane UI), um die Datenbank zu verwalten.

- [ ] **Frontend Grundgerüst & Routing**
  - [x] Basis HTML/CSS/JS (Dark/Arcane Theme).
  - [x] Tab-Navigation (Dashboard, Hosts, Profiles, Modules, Templates).
  - [ ] Tab-Navigation für "Raw Files" ergänzen.
- [ ] **Host Management UI**
  - [x] Tabelle zur Anzeige aller Hosts.
  - [ ] "Add Host" Modal/Formular (Eingabefelder für Hostname, IP, Systemarchitektur).
  - [ ] Toggle für `remote_deploy` (Local vs Remote).
  - [ ] Multi-Select Dropdowns, um Profiles und Modules mit einem Host zu verknüpfen (API: `host_add_profile`, `host_add_option`).
- [ ] **Profile, Module & Template Management UI**
  - [ ] Formulare zum Erstellen von Profilen (Name, Pfad).
  - [ ] Code-Editor-Komponente (z.B. Monaco Editor oder CodeMirror) zum Bearbeiten der rohen `.nix` Inhalte für Module und Templates im Browser.
- [ ] **Raw Files Management UI**
  - [ ] Drag & Drop Feld für Datei-Uploads (z.B. `.age` Secrets).
  - [ ] Binär-Upload-Logik in JavaScript (`fetch` mit rohen Bytes).

## Phase 3: Config Ansicht Interface (Read-Only)
Die Möglichkeit, die geplante oder generierte Config im Browser zu überprüfen.

- [ ] **Nix File Viewer**
  - [ ] Baumstruktur (Tree View) der generierten Dateien anzeigen (ähnlich einem IDE-Explorer).
  - [ ] Syntax-Highlighting für Nix im Browser.
  - [ ] Anzeige von Raw-Textdateien (Python-Skripte etc.), Ausblenden von Binärdaten (Hex-Dump oder Warnung "Binary File").
- [ ] **Diff View**
  - [ ] Visualisierung der Änderungen (Was wird hinzugefügt/gelöscht), bevor der Build gestartet wird.

## Phase 4: Config Rebuild Ansicht & Git Automatisierung
Der Übergang von der Datenbank in das Dateisystem und die Versionskontrolle.

- [ ] **Der "Generate Config" Workflow**
  - [x] `generate_config` Endpunkt im Backend (schreibt VFS auf Festplatte).
  - [x] Schreiben der Raw-Files (Binärdaten 1:1 auf die Disk dumpen).
  - [ ] Frontend: "Apply Changes" Button, der den Endpunkt triggert.
- [ ] **Git & Versionskontrolle**
  - [ ] Nach `std::fs::write` in der Generate-Schleife: Automatisches `git add .`.
  - [ ] Automatisches `git commit -m "chore: auto-generated config update"`.
- [ ] **Nix Flake Check & Rollback**
  - [ ] Führe `nix flake check` oder `nixos-rebuild dry-build` nach dem Commit aus.
  - [ ] Bei Fehler: Automatisches `git reset --hard HEAD~1` (Rollback) und Fehlermeldung via SSE an das Frontend senden.

## Phase 5: Remote Deployment Status & Installation
Das tatsächliche Ausrollen der Config auf die Server.

- [ ] **Deployment Backend**
  - [x] Colmena-Integration / `nixos-rebuild` Remote Ausführung.
  - [ ] Streaming der Build-Logs (stdout/stderr) über das bestehende SSE (`event_stream`) in die Console-Komponente des Web-Dashboards.
- [ ] **Rebuild Status UI**
  - [ ] Live-Fortschrittsbalken pro Host während des Deployments.
  - [ ] Matrix-Multiplikation-Screensaver / "Xanterella" Hacker-Text-Animation (während lange Builds laufen).
  - [ ] Erfolgs-/Fehler-Indikatoren (Grüner Haken / Rotes Kreuz) pro Node am Ende des Deployments.

## Phase 6: Caching & Attic Nix Binary Cache
Optimierung der Build-Zeiten für dein Homelab.

- [ ] **Attic Integration**
  - [ ] Automatischer Push der gebauten Derivations in deinen Attic-Server nach erfolgreichem Build.
  - [ ] Generierung der `nix.settings.substituters` und `nix.settings.trusted-public-keys` für alle Hosts in der `hosts.nix`, um den Attic Cache zu nutzen.
- [ ] **Caching UI**
  - [ ] Anzeige der Cache-Hit-Rate im Dashboard.
  - [ ] Button zum manuellen Clearen/Verwalten des Binary Caches.

## Phase 7: Homelabbing Services & Uptime Monitor
Direkte System-Integration und Überwachung.

- [ ] **Uptime Monitor / Heartbeat**
  - [x] Ping & SSH-Ping Endpunkte (`/ping/:ip`).
  - [ ] CRON-Job / Hintergrund-Task im Backend, der alle x Minuten alle Hosts aus der Datenbank anpingt.
  - [ ] Dashboard-Integration: Status-Ampel (Grün/Gelb/Rot) für jeden Host basierend auf der letzten Heartbeat-Antwort.
- [ ] **Direkte Config Integration**
  - [ ] Auslesen spezifischer Services (z.B. läuft ein Webserver, Docker, etc.) aus der generierten Config und Anzeige im Dashboard als "Active Services".

## Phase 8: Daemon & Auto-Init (Sidequests / Future)
USB-Automatisierung und Initial-Setup für neue Bare-Metal Server.

- [ ] **Init Setup**
  - [ ] Automatisches Befüllen der `xanterella.db` bei initialem Start aus einer leeren Umgebung.
  - [ ] Injektion der Base-Configs (WLAN, Tailscale) via API.
- [ ] **USB Daemon**
  - [ ] Hintergrundprozess, der einen eingesteckten USB-Stick erkennt.
  - [ ] Automatisches Flashen einer NixOS-ISO mit vorab injiziertem Tailscale-Key zur Remote-Installation (Zero-Click Provisioning).
- [ ] **CI/CD Testing**
  - [ ] Ephemeral Integration Testing (Starten der generierten Config in einer temporären QEMU-VM über GitHub Actions, bevor es aufs Homelab deployed wird).
