use std::{fs, path::Path};

pub fn write_flake(root: &Path) {
    for (name, content) in [
        ("flake.nix", "{ description = \"test configuration\"; }"),
        ("flake-modules/packages.nix", "{}"),
        ("flake-modules/hosts.nix", "{}"),
        (
            "flake-modules/colmena-hosts.nix",
            "{ worker = { imports = [ ../hosts/worker/configuration.nix ]; deployment = { buildOnTarget = true; }; }; }",
        ),
        ("hosts/worker/configuration.nix", "{ imports = [ ./hardware-configuration.nix ]; }"),
        ("hosts/worker/hardware-configuration.nix", "{}"),
        ("profiles/apps/worker.nix", "{ config = { xanterella = { enabled = true; }; }; }"),
        ("modules/tools/editor.nix", "{}"),
        (".gitignore", "*.tmp"),
        ("flake.lock", "{}"),
    ] {
        let path = root.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }
    fs::write(root.join("icon.bin"), [255, 0, 1]).unwrap();
}
