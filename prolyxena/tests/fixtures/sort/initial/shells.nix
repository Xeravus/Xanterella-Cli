{
  config,
  pkgs,
  lib,
  inputs,
  ...
}: let
  p10kConf = ./p10k.zsh;
  yaziFunc = ''
    function y() {
      local tmp="$(mktemp -t "yazi-cwd.XXXXXX")" cwd
      command yazi "$@" --cwd-file="$tmp"
      IFS= read -r -d "" cwd < "$tmp"
      [ "$cwd" != "$PWD" ] && [ -d "$cwd" ] && builtin cd -- "$cwd"
      rm -f -- "$tmp"
    }
  '';
  userZshrc = pkgs.writeText "zshrc" ''
    # 1. Cache & Instant Prompt (prompt_cr ist hier bereits durch NixOS deaktiviert!)
    ZSH_CACHE_DIR="$HOME/.cache/zsh"
    if [[ ! -d "$ZSH_CACHE_DIR" ]]; then
      mkdir -p "$ZSH_CACHE_DIR"
    fi
    export ZSH_COMPDUMP="$ZSH_CACHE_DIR/zcompdump-$HOST-$ZSH_VERSION"
    export DIRENV_LOG_FORMAT=""

    if [[ -r "''${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-''${(%):-%n}.zsh" ]]; then
      source "''${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-''${(%):-%n}.zsh"
    fi

    # 2. Theme laden (direkt aus den Flake Inputs)
    source ${inputs.p10k-src}/powerlevel10k.zsh-theme

    # 3. Config laden
    source ${p10kConf}

    # 4. Yazi anhängen
    ${yaziFunc}

    # 5. Sauberer Abschluss (Exakt am Ende der Datei, genau wie P10k es verlangt!)
    (( ! ''${+functions[p10k]} )) || p10k finalize
  '';
in {
    options = {
      xanterella = {
        zsh = {
          enable = lib.mkEnableOption "Aktiviert zsh";
        };
        bash = {
          enable = lib.mkEnableOption "Aktiviert Bash";
        };
      };
    };
    config = lib.mkMerge [
      (lib.mkIf config.xanterella.zsh.enable {
        environment = {
          systemPackages = with pkgs; [
            zsh-powerlevel10k
            bat
          ];
        };
        systemd = {
          user = {
            tmpfiles = {
              rules = [
                "L+ %h/.zshrc - - - - ${userZshrc}"
              ];
            };
          };
        };
        users = {
          users = {
            cato = {
              shell = pkgs.zsh;
            };
            root = {
              shell = pkgs.zsh;
            };
          };
        };
        programs = {
          zsh = {
            enable = true;
            enableCompletion = true;
            enableBashCompletion = true;
            enableLsColors = true;
            autosuggestions = {
              enable = true;
            };
            syntaxHighlighting = {
              enable = true;
            };
            promptInit = "";
            setOptions = [
              "NO_NOMATCH"
              "NO_PROMPT_CR"
            ];
            shellAliases = {
              l = "ls -lha";
              cl = "clear";
              f = "fastfetch";
              v = "nvim";
              vim = "nvim";
              sv = "sudo nvim";
              za = "yazi";
              nix-pr = "nixpkgs-review pr --print-result";
              b = "btop";
              carrun = "cargo c && cargo t && cargo b";
              pcl = "pyroclear";
              plc = "pyroclear";
              p = "pyroclear";
            };
            interactiveShellInit = "";
          };
        };
      })
      (lib.mkIf config.xanterella.bash.enable {
        programs = {
          bash = {
            completion = {
              enable = true;
            };
            shellAliases = {
              l = "ls -lha";
              cl = "clear";
              f = "fastfetch";
              v = "nvim";
              vim = "nvim";
              sv = "sudo nvim";
              za = "yazi";
              nix-pr = "nixpkgs-review pr --print-result";
            };
          };
        };
        users = {
          users = {
            cato = {
              shell = pkgs.bash;
            };
            root = {
              shell = pkgs.bash;
            };
          };
        };
      })
    ];
  }