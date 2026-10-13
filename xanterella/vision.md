# Xanterella
## 0.1v Xanterella - Remote-Install
- [x] Tailscale Device Fetch
- [x] Remote Partitioning
- [x] Remote Mounting
- [ ] Remote Install

## 0.2v Xanterella - USB
- [ ] USB Flashing
- [ ] Remote USB Flashing

## 0.3v Xanterella - Daemon
- [ ] Basic USB Discovery Daemon
- [ ] Remote-Install Deamon

## 0.4v Xanterella - Auto Init
### 0.41v Xanterella - Auto Init - Injection
- [ ] Generall Config File
- [ ] Wlan Injection
- [ ] Tailscale Key Injection

### 0.42v Xanterella - Auto Init - Git/GitHub/Tailscale
- [ ] Git Init
- [ ] GitHub Init
- [ ] Tailscale Init

## 0.5v Xanterella - Config -> Prolyxena
- [x] Basic sort Host
- [x] Basic add Host
    - [x] Basic remove Host
    - [ ] Basic diff Host feature
- [ ] Basic list Modules for Host

## 0.6v Xanterella - TUI
- [ ] From CLI to TUI

## 0.7v Xanterella - Extended Daemon
- [ ] Advanced Daemon
    - [ ] Approval Flow(with PR(maybe))
    - [ ] Remote Builder
- [ ] Auto add Host
    - [ ] Add Host in hosts/
    - [ ] Add Host in Colmena-Hosts
    - [ ] Auto colmena deploy after install

# Sidequests
## CiCd Pipelin
### Ci Part - Zero-Click Tests
- [x] - Self Hosted Github Runner
    - [x] - Deklaritiv selfhosted GitHub runner
- [x] - GitHub Action
    - [x] - On Pull Request
        - [x] - Multiple Scipts for different branches
### Cd Part - Zero-Click Release
- [x] - Self hosted GitHub Runner
    - [x] - Deklaritiv selfhosted GitHub runner
- [x] - GitHub Action
    - [ ] - On Pull Request / Tag
        - [ ] - Tests
        - [ ] - Release 
            - [ ] - Autocompiled Binaries
                - [ ] - Cross compiled
                    - [ ] - x86_64
                    - [ ] - aarch64
### Ephermeral Integration Testing (Testing of the installer virtualy using QEMU)
- [x] - Self hosted GitHub Runner
    - [ ] - Deklaritiv selfhosted GitHub runner
- [ ] - GitHub Action
