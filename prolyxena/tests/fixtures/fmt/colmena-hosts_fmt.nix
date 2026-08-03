{
  commonSSHKeys,
  inputs,
  pkgs-new,
  pkgs-unstable,
  systemarch,
  taruser,
  ...
}: {
  lutik = {
    deployment = {
      buildOnTarget = false;
      keys = commonSSHKeys;
      targetHost = "192.168.178.34";
      targetUser = taruser;
    };
    imports = [
      ./hosts/lutik/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };
  meta = {
    nixpkgs = import inputs.nixpkgs {
      config = {
        allowUnfree = true;
      };
      purity = "impure";
      system = systemarch;
    };
    nodeNixpkgs = {
      vicuna = import inputs.nixpkgs {
        config = {
          allowUnfree = true;
        };
        system = "aarch64-linux";
      };
    };
    specialArgs = {
      inputs = inputs;
      pkgs-new = pkgs-new;
      pkgs-unstable = pkgs-unstable;
    };
  };
  vicuna = {
    deployment = {
      buildOnTarget = false;
      keys = commonSSHKeys;
      targetHost = "192.168.178.30";
      targetUser = taruser;
    };
    imports = [
      ./hosts/vicuna/configuration.nix
      ./profiles/ssh-keys.nix
      inputs.nixos-hardware.nixosModules.raspberry-pi-5
    ];
  };
  xeravus = {
    deployment = {
      allowLocalDeployment = true;
      buildOnTarget = true;
      targetHost = null;
    };
    imports = [
      ./hosts/xeravus/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };
}