{ description = "Meine NixOs Systeme"; inputs = { nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-25.11"; flake = true; }; nixpkgs-new = { url = "github:nixos/nixpkgs/nixos-26.05"; flake = true; }; nixpkgs-unstable = { url = "github:nixos/nixpkgs/nixpkgs-unstable"; flake = true; };





    nixos-hardware = {
url = "github:NixOS/nixos-hardware/master";
      flake = true;
    };
    nix-programs = {
      url = "github:Xeravus/Nixos_programs/stable";
      flake = true;
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      flake = true;
    };
    wrapper-modules = {
      url = "github:BirdeeHub/nix-wrapper-modules";
      flake = true;
    };
    spicetify-nix = {
      url = "github:Gerg-L/spicetify-nix";
      flake = true;
    };
    zen-browser = {
      url = "github:0xc000022070/zen-browser-flake";
      flake = true;
    };
    nixvim = {
      url = "github:nix-community/nixvim/nixos-25.11";
      flake = true;
    };
    alejandra = {
      url = "github:kamadorueda/alejandra/4.0.0";
      flake = true;
      inputs.nixpkgs.follows = "nixpkgs";
    };
    noctalia = {
      url = "github:noctalia-dev/noctalia";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    disko = {
      url = "github:nix-community/disko";
      flake = true;
      inputs.nixpkgs.follows = "nixpkgs";
    };
    agenix = {
      url = "github:ryantm/agenix";
      flake = true;
					};
    colmena = {
      url = "github:zhaofengli/colmena";
      flake = true;
    };
    p10k-src = {
      url = "github:romkatv/powerlevel10k";
      flake = false;
    };
    wallpaper = {
      url = "github:Xeravus/Xanterella-Etc";
      flake = false;
    };
    pomo-src = {
      url = "github:Bahaaio/pomo";
      flake = false;
    };
  };
  outputs = inputs @ {flake-parts, ...}: let
    systemarch = "x86_64-linux";
    taruser = "root";
    commonSSHKeys = {
      "id_ed25519" = {
        keyFile = "/home/cato/.ssh/id_ed25519";
        destDir = "/etc/ssh";
        user = "root";
        group = "root";
        permissions = "0600";
      };
      "github_key" = {
        keyFile = "/home/cato/.ssh/id_github";
        destDir = "/root/.ssh";
        user = "root";
        permissions = "0600";
      };
    };
  in
    flake-parts.lib.mkFlake {
      inputs = inputs;
    } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      imports = [
        ./flake-modules/hosts.nix
        ./flake-modules/colmena.nix ./flake-modules/dev-shells.nix ]; perSystem = { pkgs, system,
        ...
      }: {};
    };
}
