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
  vicuna = {
    deployment = {
      targetHost = "192.168.178.30";
      targetUser = taruser;
      buildOnTarget = false;
      keys = commonSSHKeys;
    };
    imports = [
      ./hosts/vicuna/configuration.nix
      ./profiles/ssh-keys.nix
      inputs.nixos-hardware.nixosModules.raspberry-pi-5
    ];
  };
  lutik = {
    deployment = {
      targetHost = "192.168.178.34";
      targetUser = taruser;
      buildOnTarget = false;
      keys = commonSSHKeys;
    };
    imports = [
      ./hosts/lutik/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };
  # --- Xanterella Hosts End ---
}
