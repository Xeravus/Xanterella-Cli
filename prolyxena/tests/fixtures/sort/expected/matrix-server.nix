{
  config,
  lib,
  pkgs,
  pkgs-unstable,
  ...
}: {
  config = lib.mkIf config.xanterella.matrix-server.enable {
    age = {
      secrets = {
        discord_secrets = {
          file = ./../agenix/mautrix_disord.env.age;
          group = "mautrix-discord";
          owner = "mautrix-discord";
        };
        matrix-password = {
          file = ./../agenix/matrix.yaml.age;
          group = "matrix-synapse";
          owner = "matrix-synapse";
        };
        whatsapp_secrets = {
          file = ./../agenix/mautrix_whatsapp.env.age;
          group = "mautrix-whatsapp";
          owner = "mautrix-whatsapp";
        };
      };
    };
    nixpkgs = {
      config = {
        permittedInsecurePackages = [
          "olm-3.2.16"
        ];
      };
    };
    services = {
      caddy = {
        enable = true;
        virtualHosts = {
          "https://${config.xanterella.matrix-server.domain}" = {
            extraConfig = ''
              handle /_matrix* {
              reverse_proxy 127.0.0.1:8008
              }
              handle /_synapse/client* {
              reverse_proxy 127.0.0.1:8008
              }
            '';
          };
        };
      };
      matrix-synapse = {
        enable = true;
        extraConfigFiles = [
          config.age.secrets.matrix-password.path
        ];
        settings = {
          database = {
            allow_unsafe_locale = true;
            args = {
              database = "matrix-synapse";
              host = "/run/postgresql";
              user = "matrix-synapse";
            };
            name = "psycopg2";
          };
          enable_registration = false;
          server_name = config.xanterella.matrix-server.domain;
        };
      };
      mautrix-discord = {
        enable = true;
        environmentFile = config.age.secrets.discord_secrets.path;
        settings = {
          appservice = {
            as_token = "$MAUTRIX_DISCORD_APPSERVICE_AS_TOKEN";
            database = {
              type = "postgres";
              uri = "postgres://mautrix-discord@/mautrix-discord?host=/run/postgresql";
            };
            hs_token = "$MAUTRIX_DISCORD_APPSERVICE_HS_TOKEN";
          };
          bridge = {
            permissions = {
              "@xeravus:${config.xanterella.matrix-server.domain}" = "admin";
            };
          };
          homeserver = {
            address = "http://127.0.0.1:8008";
            domain = config.xanterella.matrix-server.domain;
          };
        };
      };
      mautrix-whatsapp = {
        enable = true;
        environmentFile = config.age.secrets.whatsapp_secrets.path;
        package = pkgs-unstable.mautrix-whatsapp;
        settings = {
          appservice = {
            as_token = "$MAUTRIX_WHATSAPP_APPSERVICE_AS_TOKEN";
            hs_token = "$MAUTRIX_WHATSAPP_APPSERVICE_HS_TOKEN";
          };
          bridge = {
            permissions = {
              "@xeravus:${config.xanterella.matrix-server.domain}" = "admin";
            };
          };
          database = {
            type = "postgres";
            uri = "postgres://mautrix-whatsapp@/mautrix-whatsapp?host=/run/postgresql";
          };
          homeserver = {
            address = "http://127.0.0.1:8008";
            domain = config.xanterella.matrix-server.domain;
          };
        };
      };
      postgresql = {
        enable = true;
        ensureDatabases = [
          "matrix-synapse"
          "mautrix-discord"
          "mautrix-whatsapp"
        ];
        ensureUsers = [
          {
            ensureDBOwnership = true;
            name = "matrix-synapse";
          }
          {
            ensureDBOwnership = true;
            name = "mautrix-discord";
          }
          {
            ensureDBOwnership = true;
            name = "mautrix-whatsapp";
          }
        ];
      };
      tailscale = {
        permitCertUid = "caddy";
      };
    };
    users = {
      users = {
        caddy = {
          extraGroups = [
            "tailscale"
          ];
        };
      };
    };
  };
  options = {
    xanterella = {
      matrix-server = {
        domain = lib.mkOption {
          default = "xanterella.de/matrix";
          type = lib.types.str;
        };
        enable = lib.mkEnableOption "Aktiviert Matrix Pipeline";
      };
    };
  };
}