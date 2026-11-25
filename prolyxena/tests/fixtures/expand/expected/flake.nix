{
  description = "Meine NixOs Systeme";
  inputs = {
    agenix = {
      flake = true;
      url = "github:ryantm/agenix";
    };
    alejandra = {
      flake = true;
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
      url = "github:kamadorueda/alejandra/4.0.0";
    };
    colmena = {
      flake = true;
      url = "github:zhaofengli/colmena";
    };
    disko = {
      flake = true;
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
      url = "github:nix-community/disko";
    };
    flake-parts = {
      flake = true;
      url = "github:hercules-ci/flake-parts";
    };
    nix-programs = {
      flake = true;
      url = "github:Xeravus/Nixos_programs/stable";
    };
    nixos-hardware = {
      flake = true;
      url = "github:NixOS/nixos-hardware/master";
    };
    nixpkgs = {
      flake = true;
      url = "github:nixos/nixpkgs/nixos-25.11";
    };
    nixpkgs-bleeding-edge = {
      flake = true;
      url = "github:nixos/nixpkgs/nixpkgs-unstable";
    };
    nixpkgs-new = {
      flake = true;
      url = "github:nixos/nixpkgs/nixos-26.05";
    };
    nixpkgs-unstable = {
      flake = true;
      url = "github:nixos/nixpkgs/nixpkgs-unstable";
    };
    nixvim = {
      flake = true;
      url = "github:nix-community/nixvim/nixos-25.11";
    };
    noctalia = {
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
      url = "github:noctalia-dev/noctalia";
    };
    p10k-src = {
      flake = false;
      url = "github:romkatv/powerlevel10k";
    };
    pomo-src = {
      flake = false;
      url = "github:Bahaaio/pomo";
    };
    pyroclear = {
      flake = true;
      url = "github:shreyanth-sureshkrishnaa/pyroclear";
    };
    spicetify-nix = {
      flake = true;
      url = "github:Gerg-L/spicetify-nix";
    };
    wrapper-modules = {
      flake = true;
      url = "github:BirdeeHub/nix-wrapper-modules";
    };
    xanterella-etc = {
      flake = false;
      url = "github:Xeravus/Xanterella-Etc";
    };
    zen-browser = {
      flake = true;
      url = "github:0xc000022070/zen-browser-flake";
    };
  };
  outputs = inputs @ {
    flake-parts,
    ...
  }: let
    commonSSHKeys = {
      "github_key" = {
        destDir = "/root/.ssh";
        keyFile = "/home/cato/.ssh/id_github";
        permissions = "0600";
        user = "root";
      };
      "id_ed25519" = {
        destDir = "/etc/ssh";
        group = "root";
        keyFile = "/home/cato/.ssh/id_ed25519";
        permissions = "0600";
        user = "root";
      };
    };
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