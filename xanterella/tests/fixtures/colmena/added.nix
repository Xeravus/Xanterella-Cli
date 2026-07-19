{
  inputs,
  systemarch,
  taruser,
  commonSSHKeys,
  pkgs-new,
  pkgs-unstable,
  ...
}: {
  meta = {
    nixpkgs = import inputs.nixpkgs {
      system = systemarch;
      config = {
        allowUnfree = true;
      };
      purity = "impure";
    };
    nodeNixpkgs = {
      vicuna = import inputs.nixpkgs {
        system = "aarch64-linux";
        config = {
          allowUnfree = true;
        };
      };
    };
    specialArgs = {inherit inputs pkgs-new pkgs-unstable;};
  };
  # --- Xanterella Hosts Start ---
  lutik = {
    deployment = {
      targetHost = "192.168.178.34";
      keys = commonSSHKeys;
      buildOnTarget = false;
    };
    imports = [
      ./hosts/lutik/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };

  prolyxena = {
    deployment = {
      targetHost = "1.1.1.1";
      keys = commonSSHKeys;
      buildOnTarget = false;
    };
    imports = [
      ./hosts/prolyxena/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };

  vicuna = {
    deployment = {
      targetHost = "192.168.178.30";
      keys = commonSSHKeys;
      buildOnTarget = false;
    };
    imports = [
      ./hosts/vicuna/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };

  xeravus = {
    deployment = {
      targetHost = null;
      allowLocalDeployment = true;
      buildOnTarget = true;
    };
    imports = [
      ./hosts/xeravus/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };
  # --- Xanterella Hosts End ---
}
