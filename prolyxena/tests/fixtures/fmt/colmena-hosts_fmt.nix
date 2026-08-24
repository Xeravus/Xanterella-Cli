{
  commonSSHKeys,
  inputs,
  pkgs-new,
  pkgs-unstable,
  systemarch,
  taruser,
  ...
} @ alias: let
  gonen = gonnen;
in {
    add = b + a;
    concat = b ++ a;
    divie = b / / a;
    empty_lambda = ${{ }:  ...
}: { }};
    emptyattrset = { };
    emptyindstr = '' '';
    emptylist = [ ];
    equal = b == a;
    float = 0.1;
    group = (test {
      a = b;
      b = a;
    });
    indstr = ''      whdhwdlwadjwadjwaj ${{
  a,
  b,
  c,
  ...
}: {
  a = b;
  b = a;
}}'';
    int = 1;
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
    merge = b // a;
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
    sub = b - a;
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
    with = with pkgs; [ ];
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