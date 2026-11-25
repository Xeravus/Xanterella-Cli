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
    specialArgs = {
      inputs = inputs;
      pkgs-new = pkgs-new;
      pkgs-unstable = pkgs-unstable;
    };
  };
  xeravus = {
    deployment = {
      targetHost = null;
      allowLocalDeployment = true;
      buildOnTarget = true;
    };
    imports = [
      ./../hosts/xeravus/configuration.nix
      ./../profiles/ssh-keys.nix
    ];
  };
  xorus = {
    deployment = {
      targetHost = "192.168.178.69";
      targetUser = taruser;
      buildOnTarget = false;
      keys = commonSSHKeys;
    };
    imports = [
      ./../hosts/xorus/configuration.nix
      ./../profiles/ssh-keys.nix
    ];
  };
  lutik = {
    deployment = {
      targetHost = "lutik";
      targetUser = taruser;
      buildOnTarget = false;
      keys = commonSSHKeys;
    };
    imports = [
      ./../hosts/lutik/configuration.nix
      ./../profiles/ssh-keys.nix
    ];
  };
}
