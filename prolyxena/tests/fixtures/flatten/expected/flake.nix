{
  description = "Meine NixOs Systeme";
  inputs.agenix.flake = true;
  inputs.agenix.url = "github:ryantm/agenix";
  inputs.alejandra.flake = true;
  inputs.alejandra.inputs.nixpkgs.follows = "nixpkgs";
  inputs.alejandra.url = "github:kamadorueda/alejandra/4.0.0";
  inputs.colmena.flake = true;
  inputs.colmena.url = "github:zhaofengli/colmena";
  inputs.disko.flake = true;
  inputs.disko.inputs.nixpkgs.follows = "nixpkgs";
  inputs.disko.url = "github:nix-community/disko";
  inputs.flake-parts.flake = true;
  inputs.flake-parts.url = "github:hercules-ci/flake-parts";
  inputs.nix-programs.flake = true;
  inputs.nix-programs.url = "github:Xeravus/Nixos_programs/stable";
  inputs.nixos-hardware.flake = true;
  inputs.nixos-hardware.url = "github:NixOS/nixos-hardware/master";
  inputs.nixpkgs-bleeding-edge.flake = true;
  inputs.nixpkgs-bleeding-edge.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  inputs.nixpkgs-new.flake = true;
  inputs.nixpkgs-new.url = "github:nixos/nixpkgs/nixos-26.05";
  inputs.nixpkgs-unstable.flake = true;
  inputs.nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  inputs.nixpkgs.flake = true;
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
  inputs.nixvim.flake = true;
  inputs.nixvim.url = "github:nix-community/nixvim/nixos-25.11";
  inputs.noctalia.inputs.nixpkgs.follows = "nixpkgs";
  inputs.noctalia.url = "github:noctalia-dev/noctalia";
  inputs.p10k-src.flake = false;
  inputs.p10k-src.url = "github:romkatv/powerlevel10k";
  inputs.pomo-src.flake = false;
  inputs.pomo-src.url = "github:Bahaaio/pomo";
  inputs.pyroclear.flake = true;
  inputs.pyroclear.url = "github:shreyanth-sureshkrishnaa/pyroclear";
  inputs.spicetify-nix.flake = true;
  inputs.spicetify-nix.url = "github:Gerg-L/spicetify-nix";
  inputs.wrapper-modules.flake = true;
  inputs.wrapper-modules.url = "github:BirdeeHub/nix-wrapper-modules";
  inputs.xanterella-etc.flake = false;
  inputs.xanterella-etc.url = "github:Xeravus/Xanterella-Etc";
  inputs.zen-browser.flake = true;
  inputs.zen-browser.url = "github:0xc000022070/zen-browser-flake";
  outputs = inputs @ {
    flake-parts,
    ...
  }: let
    commonSSHKeys."github_key".destDir = "/root/.ssh";
    commonSSHKeys."github_key".keyFile = "/home/cato/.ssh/id_github";
    commonSSHKeys."github_key".permissions = "0600";
    commonSSHKeys."github_key".user = "root";
    commonSSHKeys."id_ed25519".destDir = "/etc/ssh";
    commonSSHKeys."id_ed25519".group = "root";
    commonSSHKeys."id_ed25519".keyFile = "/home/cato/.ssh/id_ed25519";
    commonSSHKeys."id_ed25519".permissions = "0600";
    commonSSHKeys."id_ed25519".user = "root";
    systemarch = "x86_64-linux";
    taruser = "root";
  in flake-parts.lib.mkFlake {
        inputs = inputs;
      } {
      imports = [
        ./flake-modules/hosts.nix
        ./flake-modules/colmena.nix
        ./flake-modules/dev-shells.nix
      ];
      perSystem = {
        pkgs,
        system,
        ...
      }: { };
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    };
}