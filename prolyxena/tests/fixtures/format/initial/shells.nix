{
  config,
  pkgs,
  lib,
  inputs,
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
              "f %h/.zshrc 0644 - - - #"
            ];
          };
        };
      };
      users = {
        defaultUserShell = pkgs.zsh;
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
          setOptions = [
            "NO_NOMATCH"
            "NO_PROMPT_CR"
          ];
          syntaxHighlighting = {
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
            b = "btop";
            carrun = "cargo c && cargo t && cargo b";
            pclear = "pyroclear";
            pcl = "pyroclear";
            plc = "pyroclear";
          };
          interactiveShellInit = ''
            unsetopt prompt_cr
                            ${zshInit}
                            source ${inputs.p10k-src}/powerlevel10k.zsh-theme
                            source ${p10kConf}
                            ${yaziFunc}

                     (( ! ''${+functions[p10k]} )) || p10k finalize
          '';
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
