{
  config,
  inputs,
  lib,
  pkgs-bleeding,
  ...
}: let
  secrets = import "/home/cato/xanterella/config/modules/agenix/usb-secrets.nix";
in {
    config = lib.mkMerge [
      (lib.mkIf config.xanterella.tailscale.enable {
        environment.systemPackages = with pkgs-bleeding; [
          tailscale
        ];
        networking.nameservers = [
          "100.100.100.100"
          "1.1.1.1"
          "1.0.0.1"
          "8.8.8.8"
        ];
        networking.search = [
          "gute-nessie.ts.net"
        ];
        services.resolved.enable = true;
        services.tailscale.enable = true;
      })
      (lib.mkIf config.xanterella.tailscale-crylia.enable {
        environment.systemPackages = with inputs.pkgs-bleeding; [
          tailscale
        ];
        networking.nameservers = [
          "100.100.100.100"
          "1.1.1.1"
          "1.0.0.1"
          "8.8.8.8"
        ];
        networking.search = [
          "gute-nessie.ts.net"
        ];
        services.resolved.enable = true;
        services.tailscale.authKeyFile = "/etc/tailscale_key";
        services.tailscale.enable = true;
        services.tailscale.extraUpFlags = [
          "--hostname=crylia"
          "--reset"
        ];
      })
      (lib.mkIf config.xanterella.tailscale-installer.enable {
        environment.etc."tailscale.key".mode = "0400";
        environment.etc."tailscale.key".text = secrets.tailscalekey;
        environment.systemPackages = with inputs.pkgs-bleeding; [
          tailscale
        ];
        services.tailscale.authKeyFile = "/etc/tailscale.key";
        services.tailscale.enable = true;
        services.tailscale.extraUpFlags = [
          "--hostname=installer"
          "--reset"
        ];
      })
    ];
    options.xanterella.tailscale.enable = lib.mkEnableOption "Aktiviert tailscale";
    options.xanterella.tailscale-crylia.enable = lib.mkEnableOption "Aktiviert tailscale für Crylia";
    options.xanterella.tailscale-installer.enable = lib.mkEnableOption "Aktiviert Tailscale für dne Installer mit autoconnect";
  }