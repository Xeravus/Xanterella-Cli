{
  config,
  inputs,
  lib,
  pkgs,
  zsh-src,
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
  zshInit = ''
    unsetopt prompt_cr prompt_sp
        ZSH_CACHE_DIR="$HOME/.cache/zsh"
        if [[ ! -d "$ZSH_CACHE_DIR" ]]; then
          mkdir -p "$ZSH_CACHE_DIR"
        fi
        export ZSH_COMPDUMP="$ZSH_CACHE_DIR/zcompdump-$HOST-$ZSH_VERSION"
        export DIRENV_LOG_FORMAT=""

        if [[ -r "''${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-''${(%):-%n}.zsh" ]]; then
          source "''${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-''${(%):-%n}.zsh"
        fi
  '';
in {
    config = lib.mkMerge [
      (lib.mkIf config.xanterella.zsh.enable {
        environment = {
          systemPackages = with pkgs; [
            zsh-powerlevel10k
            bat
          ];
        };
        programs = {
          zsh = {
            autosuggestions = {
              enable = true;
            };
            enable = true;
            enableBashCompletion = true;
            enableCompletion = true;
            enableLsColors = true;
            interactiveShellInit = ''
            unsetopt prompt_cr
                            ${zshInit}
                            source ${inputs.p10k-src}/powerlevel10k.zsh-theme
                            source ${p10kConf}
                            ${yaziFunc}

                     (( ! ''${+functions[p10k]} )) || p10k finalize
          '';
            setOptions = [
              "NO_NOMATCH"
              "NO_PROMPT_CR"
            ];
            shellAliases = {
              b = "btop";
              carrun = "cargo c && cargo t && cargo b";
              cl = "clear";
              f = "fastfetch";
              l = "ls -lha";
              nix-pr = "nixpkgs-review pr --print-result";
              pcl = "pyroclear";
              pclear = "pyroclear";
              plc = "pyroclear";
              sv = "sudo nvim";
              v = "nvim";
              vim = "nvim";
              za = "yazi";
            };
            syntaxHighlighting = {
              enable = true;
            };
          };
        };
        systemd = {
          user = {
            tmpfiles = {
              rules = [
                "f %h/.zshrc 0644 - - - #"
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
      })
      (lib.mkIf config.xanterella.bash.enable {
        programs = {
          bash = {
            completion = {
              enable = true;
            };
            shellAliases = {
              cl = "clear";
              f = "fastfetch";
              l = "ls -lha";
              nix-pr = "nixpkgs-review pr --print-result";
              sv = "sudo nvim";
              v = "nvim";
              vim = "nvim";
              za = "yazi";
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
    options = {
      xanterella = {
        bash = {
          enable = lib.mkEnableOption "Aktiviert Bash";
        };
        zsh = {
          enable = lib.mkEnableOption "Aktiviert zsh";
        };
      };
    };
  }