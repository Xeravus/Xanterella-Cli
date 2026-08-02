{
  config,
  pkgs-unstable,
  lib,
  ...
}: let
  secrets = import "/home/cato/xanterella/config/modules/agenix/usb-secrets.nix";
in {
  options = {
    xanterella = {
      tailscale = {
        enable = lib.mkEnableOption "Aktiviert tailscale";
      };
      tailscale-crylia = {
        enable = lib.mkEnableOption "Aktiviert tailscale für Crylia";
      };
      tailscale-installer = {
        enable = lib.mkEnableOption "Aktiviert Tailscale für dne Installer mit autoconnect";
      };
    };
  };

  config = lib.mkMerge [
    (lib.mkIf config.xanterella.tailscale.enable {
      environment = {
        systemPackages = with pkgs-unstable; [
          tailscale
        ];
      };
      services = {
        tailscale = {
          enable = true;
        };
        resolved = {
          enable = true;
        };
      };
      networking = {
        nameservers = [
          "100.100.100.100"
          "1.1.1.1"
          "1.0.0.1"
          "8.8.8.8"
        ];
        search = [
          "gute-nessie.ts.net"
        ];
      };
    })
    (lib.mkIf config.xanterella.tailscale-crylia.enable {
      environment = {
        systemPackages = with pkgs-unstable; [
          tailscale
        ];
      };
      services = {
        tailscale = {
          enable = true;
          authKeyFile = "/etc/tailscale_key";
          extraUpFlags = [
            "--hostname=crylia"
            "--reset"
          ];
        };
        resolved = {
          enable = true;
        };
      };
      networking = {
        nameservers = [
          "100.100.100.100"
          "1.1.1.1"
          "1.0.0.1"
          "8.8.8.8"
        ];
        search = [
          "gute-nessie.ts.net"
        ];
      };
    })
    (lib.mkIf config.xanterella.tailscale-installer.enable {
      environment = {
        systemPackages = with pkgs-unstable; [
          tailscale
        ];
        etc = {
          "tailscale.key" = {
            text = secrets.tailscalekey;
            mode = "0400";
          };
        };
      };
      services = {
        tailscale = {
          enable = true;
          authKeyFile = "/etc/tailscale.key";
          extraUpFlags = [
            "--hostname=installer"
            "--reset"
          ];
        };
      };
    })
  ];
}
