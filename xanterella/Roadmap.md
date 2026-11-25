# Meine Rust-Roadmap für Xanterella

- [ ] **Nutze Polymorphismus, wenn du Verhalten austauschen möchtest.** Mit `Xanterella<C: CommandRunner>` kannst du denselben Ablauf mit verschiedenen Runnern ausführen. Das Trait beschreibt, was ein Runner können muss. Der Typparameter `C` legt beim Kompilieren fest, welche Implementierung zum Einsatz kommt. Nutze das im Test, um zwei Fälle gezielt nachzustellen: Tailscale kann gar nicht starten, oder es startet und meldet einen Fehler. [CommandRunner](core/src/xanterella.rs) · [get_taildevices](core/src/get.rs)

- [ ] **Nutze Generics, wenn mehrere Eingabetypen dieselbe Aufgabe erfüllen.** In `colmena_apply` kannst du mit `T: AsRef<str>` sowohl geliehene Hostnamen als auch eigene Strings lesen. Braucht eine Funktion dagegen einen eigenen Bytepuffer, passt `impl Into<Vec<u8>>`. Bei `add_raw` könnte so ein bereits vorhandener Vektor direkt an die Datenbankabfrage weitergegeben werden. [colmena_apply](core/src/nix/colmena.rs) · [add_raw](core/src/database/raw.rs)

- [ ] **Verschiebe Werte, wenn ihr bisheriger Besitzer sie nicht mehr braucht.** In `GitManager::merge(self)` gehört dir der Manager. Du kannst `repo` und `directory` genauso in den nächsten Zustand verschieben, wie du es mit `main_br` bereits machst. Frag dich vor einem `.clone()`, ob anschließend wirklich zwei Besitzer je einen eigenen Wert brauchen. [GitManager::merge](core/src/git.rs)

- [ ] **Drücke fehlende Werte mit `Option` aus.** Bei `get_taildevices_specific` kann ein Peer eine leere IP-Liste haben. Mit `device.ip.into_iter().next()` erhältst du entweder die erste Adresse oder `None`. Teste einen Peer ohne Adresse zusammen mit einem zweiten, der eine hat. So hältst du fest, dass der zweite weiterhin gefunden wird. [get_taildevices_specific](core/src/get.rs)

- [ ] **Nutze Typen, um die Reihenfolge von Operationen abzusichern.** Bei `GitManager<State>` legt der Typparameter fest, welche Methoden verfügbar sind. Dadurch lässt sich `commit()` erst auf einem `GitManager<Tracked>` aufrufen. Halte das Feld `state` privat, wenn Aufrufer diesen Zustand nur über `add()` erreichen sollen. Dieses Muster heißt *Typestate*. [GitManager](core/src/git.rs)
