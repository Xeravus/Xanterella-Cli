#[path = "../../tests/support/mod.rs"]
mod support;

use std::{fs, path::Path, process::Command};

use xanterella_core::git::{Branches, GitManager};

fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git").current_dir(dir).args(args).output().unwrap();
    assert!(output.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn identity(dir: &Path) {
    git(dir, &["config", "user.name", "Integration Test"]);
    git(dir, &["config", "user.email", "test@example.invalid"]);
    git(dir, &["config", "commit.gpgsign", "false"]);
}

#[tokio::test]
async fn publishes_a_generated_branch_to_a_local_remote() {
    let dir = support::TestDir::new();
    let remote = dir.path().join("remote.git");
    git(dir.path(), &["init", "--bare", "--initial-branch=main", "remote.git"]);
    git(dir.path(), &["clone", "remote.git", "seed"]);
    let seed = dir.path().join("seed");
    identity(&seed);
    fs::write(seed.join("flake.nix"), "{}").unwrap();
    git(&seed, &["add", "flake.nix"]);
    git(&seed, &["commit", "-m", "seed"]);
    git(&seed, &["push", "origin", "HEAD:main"]);

    let work = dir.path().join("work");
    let manager = GitManager::new(remote.display().to_string(), work.display().to_string(), "main".into()).await;
    let manager = manager.init().await.unwrap();
    identity(&work);
    let manager = manager.checkout(Branches::Xanterella).await.unwrap().merge().await.unwrap();
    fs::write(work.join("generated.nix"), "{ enabled = true; }").unwrap();
    manager.add().await.unwrap().commit("generated".into()).await.unwrap().push().await.unwrap();

    assert_eq!(git(&remote, &["show", "xanterella:generated.nix"]), "{ enabled = true; }");
    assert_eq!(git(&remote, &["diff", "--name-only", "main", "xanterella"]), "generated.nix");
}
