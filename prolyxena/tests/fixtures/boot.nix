{
  config,
  pkgs,
  lib,
  ...
}: {
  options = {
    xanterella = {
      boot = {
        enable = lib.mkEnableOption "Aktiviert Grub als bootloader";
      };
      boot-server = {
        enable = lib.mkEnableOption "Aktiviert Bootoptions für Raspberry Pi Server";
      };
    };
  };

  config = lib.mkMerge [
    (lib.mkIf config.xanterella.boot.enable {
      boot = {
        loader = {
          efi = {
            canTouchEfiVariables = true;
          };
          systemd-boot = {
            enable = true;
          };
        };
        kernelPackages = pkgs.linuxPackages_6_12;
        kernelParams = ["btusb.enable_autosuspend=0"];
      };
    })

    (lib.mkIf config.xanterella.boot-server.enable {
      boot = {
        loader = {
          efi = {
            canTouchEfiVariables = true;
          };
          systemd-boot = {
            enable = false;
          };
          grub = {
            enable = false;
          };
          generic-extlinux-compatible = {
            enable = true;
          };
        };
        kernelPackages = pkgs.linuxPackages_6_12;
        kernelParams = ["btusb.enable_autosuspend=0"];
      };
    })
  ];
}
