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
        environment.systemPackages = with pkgs; [
          zsh-powerlevel10k
          bat
        ];
        programs.zsh.autosuggestions.enable = true;
        programs.zsh.enable = true;
        programs.zsh.enableBashCompletion = true;
        programs.zsh.enableCompletion = true;
        programs.zsh.enableLsColors = true;
        programs.zsh.interactiveShellInit = ''
            unsetopt prompt_cr
                            ${zshInit}
                            source ${inputs.p10k-src}/powerlevel10k.zsh-theme
                            source ${p10kConf}
                            ${yaziFunc}

                     (( ! ''${+functions[p10k]} )) || p10k finalize
          '';
        programs.zsh.setOptions = [
          "NO_NOMATCH"
          "NO_PROMPT_CR"
        ];
        programs.zsh.shellAliases.b = "btop";
        programs.zsh.shellAliases.carrun = "cargo c && cargo t && cargo b";
        programs.zsh.shellAliases.cl = "clear";
        programs.zsh.shellAliases.f = "fastfetch";
        programs.zsh.shellAliases.l = "ls -lha";
        programs.zsh.shellAliases.nix-pr = "nixpkgs-review pr --print-result";
        programs.zsh.shellAliases.pcl = "pyroclear";
        programs.zsh.shellAliases.pclear = "pyroclear";
        programs.zsh.shellAliases.plc = "pyroclear";
        programs.zsh.shellAliases.sv = "sudo nvim";
        programs.zsh.shellAliases.v = "nvim";
        programs.zsh.shellAliases.vim = "nvim";
        programs.zsh.shellAliases.za = "yazi";
        programs.zsh.syntaxHighlighting.enable = true;
        systemd.user.tmpfiles.rules = [
          "f %h/.zshrc 0644 - - - #"
        ];
        users.users.cato.shell = pkgs.zsh;
        users.users.root.shell = pkgs.zsh;
      })
      (lib.mkIf config.xanterella.bash.enable {
        programs.bash.completion.enable = true;
        programs.bash.shellAliases.cl = "clear";
        programs.bash.shellAliases.f = "fastfetch";
        programs.bash.shellAliases.l = "ls -lha";
        programs.bash.shellAliases.nix-pr = "nixpkgs-review pr --print-result";
        programs.bash.shellAliases.sv = "sudo nvim";
        programs.bash.shellAliases.v = "nvim";
        programs.bash.shellAliases.vim = "nvim";
        programs.bash.shellAliases.za = "yazi";
        users.users.cato.shell = pkgs.bash;
        users.users.root.shell = pkgs.bash;
      })
    ];
    options.xanterella.bash.enable = lib.mkEnableOption "Aktiviert Bash";
    options.xanterella.zsh.enable = lib.mkEnableOption "Aktiviert zsh";
  }