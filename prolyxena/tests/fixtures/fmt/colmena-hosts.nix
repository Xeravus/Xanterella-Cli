{
  inputs,
  systemarch,
  taruser,
  commonSSHKeys,
  pkgs-new,
  pkgs-unstable,
  ...
} @ alias: let
gonen = gonnen;
in {
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
  emptyattrset = {};
  emptylist = [];
  emptyindstr = '''';

  indstr = ''whdhwdlwadjwadjwaj ${{b, c, a, ...}: { b=a;a=b;
  }}'';
  float = 0.1;
  int = 1;
  group = (test {b=a;a=b;});
  with = with pkgs; [
  ];
  empty_lambda = ${{ }: {}};
  add = b + a;
  sub = b - a;
  concat = b ++ a;
  equal = b == a;
  merge = b // a;
  divie = b / a;
}
