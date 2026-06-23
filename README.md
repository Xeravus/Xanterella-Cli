# Xanterella CLI

> *"Noli papilones venari, sed hortum excole, et sponte volabunt."*

Let me welcome you into my cyber-garden. **Xanterella** isn't just a CLI-Tool, it's my personal project learning. I started this project to get to know [Rust](https://www.rust-lang.org/) and start learning with a porpuse in mind, while strengthening my understanding of [Nix/NixOS](https://nixos.org/). 

Even if this project might not be used, I still got the knowledge, the fun and the expirience by developing it.

---

## *Ratio et Propositum* (Porpuse & Usecase)

Xanterella is a Command Line Interface written in Rust, designed for the **managment, remote installation, orchestration of Nix & NixOS configurations** across one or multiple hosts. 

Through this project, I have learned hot to structure CLI application in Rust, while I've also build a complete, custom CI Pipeline featuring my very own deklaritiv **self-hosted GitHub runner**.

## *Instrumenta* (Features)

While this tool is in constant development, this are the implemented features:

* **remota installatio (`remote-install`)**: Fully automatic installation(Device Check, Partitionating, Formating, Installing) from anywhere via tailscale.
* **Spiritus Modus (`daemon`)**: Fully automatic remote installing daemon which detects tailscale device with a specific tag.
* **USB Flashing (`flash`)**: Builds and flashes local or remote an usb-stick with the installer for the remote-install und daemon-install.
* **Auto Init & Injection (`init`)**: Automatic injects wifi passwords and tailscale keys into installing devices.

## *Fabricando fit faber* (Installation & Use)

The project is build using flakes and requires Nix to be installed.

```bash
# Clone repo
git clone [https://github.com/Xeravus/Xanterella-Cli.git](https://github.com/Xeravus/Xanterella-Cli.git)
cd Xanterella-Cli

# Start dev Shell (Rust, Cargo, Rust-Analyzer etc.)
nix develop .#xanterella

# Compile CLI
nix build .#xanterella
```

## *Quo Vadis?* (Roadmap)

> *"Mutatio est aeterna."*

* - [ ] **Config & Colmena Integration**: Zero-touch adding, removeing, sorting, parsing for hosts, moduls, profiles via Colmena & Flakes.
* - [ ] **TUI**: Switching from CLI to TUI for the config part
* - [ ] **Advanced Daemon**: Better features for the daemon: Remote Builder, Approval-Workflow for installations
