let
  systemarch = "x86_64-linux";
  pkgs-new = import inputs.nixpkgs-new {
    system = systemarch;
    config = {
      allowUnfree = true;
    };
  };
  taruser = "root";
  pkgs-unstable = import inputs.nixpkgs-unstable {
    system = systemarch;
    config = {
      allowUnfree = true;
    };
  };
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
in {}
