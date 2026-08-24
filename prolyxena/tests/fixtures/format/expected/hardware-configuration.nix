{
  config,
  lib,
  modulesPath,
  pkgs,
  ...
}: {
  boot = {
    extraModulePackages = [ ];
    initrd = {
      availableKernelModules = [
        "ahci"
        "nvme"
        "sd_mod"
        "uas"
        "usb_storage"
        "xhci_pci"
      ];
      kernelModules = [ ];
    };
    kernelModules = [
      "kvm-intel"
    ];
  };
  fileSystems = {
    "/" = {
      device = "/dev/disk/by-uuid/d53b5516-59f1-4ec1-a7b2-03549ebf5a59";
      fsType = "ext4";
    };
    "/boot" = {
      device = "/dev/disk/by-uuid/7548-41A2";
      fsType = "vfat";
      options = [
        "dmask=0022"
        "fmask=0022"
      ];
    };
  };
  hardware = {
    cpu = {
      intel = {
        updateMicrocode = lib.mkDefault config.hardware.enableRedistributableFirmware;
      };
    };
  };
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
  ];
  nixpkgs = {
    hostPlatform = lib.mkDefault "x86_64-linux";
  };
  swapDevices = [ ];
}