{
  description = "Flake für das Cli von Xanterella";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    naersk,
    ...
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
    rust-nightly = pkgs.rust-bin.nightly.latest.default.override {
      extensions = [
        "rustfmt"
        "clippy"
        "rust-src"
      ];
    };
    naerskLib = pkgs.callPackage naersk {
      cargo = rust-nightly;
      rustc = rust-nightly;
    };
  in {
    devShells."x86_64-linux" = {
      xanterella = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-nightly
          rust-analyzer
          tokei
          cargo-tarpaulin
          cargo-audit
          cargo-machete

          alejandra
          openssh
          util-linux
          iputils
          parted
          dosfstools
          e2fsprogs
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
      prolyxena = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-nightly
          rust-analyzer
          tokei
          cargo-tarpaulin
          cargo-audit
          cargo-machete
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    };
    packages."x86_64-linux" = {
      xanterella = naerskLib.buildPackage {
        src = ./xanterella/.;
        buildInputs = [
          pkgs.pkg-config
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
      };
      prolyxena = naerskLib.buildPackage {
        src = ./prolyxena/.;
        buildInputs = [
          pkgs.pkg-config
        ];
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
      };
    };
  };
}
