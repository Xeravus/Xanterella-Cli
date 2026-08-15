{
  config,
  lib,
  modulesPath,
  pkgs,
  ...
}: {
  boot.extraModulePackages = [ ];
  boot.initrd.availableKernelModules = [
    "xhci_pci"
    "ahci"
    "nvme"
    "usb_storage"
    "uas"
    "sd_mod"
  ];
  boot.initrd.kernelModules = [ ];
  boot.kernelModules = [
    "kvm-intel"
  ];
  fileSystems."/".device = "/dev/disk/by-uuid/d53b5516-59f1-4ec1-a7b2-03549ebf5a59";
  fileSystems."/".fsType = "ext4";
  fileSystems."/boot".device = "/dev/disk/by-uuid/7548-41A2";
  fileSystems."/boot".fsType = "vfat";
  fileSystems."/boot".options = [
    "fmask=0022"
    "dmask=0022"
  ];
  hardware.cpu.intel.updateMicrocode = lib.mkDefault config.hardware.enableRedistributableFirmware;
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
  ];
  nixpkgs.hostPlatform = lib.mkDefault "x86_64-linux";
  swapDevices = [ ];
}