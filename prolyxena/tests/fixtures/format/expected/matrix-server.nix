{
  config,
  pkgs,
  lib,
  pkgs-unstable,
  ...
}: {
  options = {
    xanterella = {
      matrix-server = {
        enable = lib.mkEnableOption "Aktiviert Matrix Pipeline";
        domain = lib.mkOption {
          type = lib.types.str;
          default = "xanterella.de/matrix";
        };
      };
    };
  };
  config = lib.mkIf config.xanterella.matrix-server.enable {
    age = {
      secrets = {
        matrix-password = {
          file = ./../agenix/matrix.yaml.age;
          owner = "matrix-synapse";
          group = "matrix-synapse";
        };
        discord_secrets = {
          file = ./../agenix/mautrix_disord.env.age;
          owner = "mautrix-discord";
          group = "mautrix-discord";
        };
        whatsapp_secrets = {
          file = ./../agenix/mautrix_whatsapp.env.age;
          owner = "mautrix-whatsapp";
          group = "mautrix-whatsapp";
        };
      };
    };
    services = {
      postgresql = {
        enable = true;
        ensureDatabases = [
          "matrix-synapse"
          "mautrix-whatsapp"
          "mautrix-discord"
        ];
        ensureUsers = [
          {
            name = "matrix-synapse";
            ensureDBOwnership = true;
          }
          {
            name = "mautrix-whatsapp";
            ensureDBOwnership = true;
          }
          {
            name = "mautrix-discord";
            ensureDBOwnership = true;
          }
        ];
      };
      matrix-synapse = {
        enable = true;
        settings = {
          server_name = config.xanterella.matrix-server.domain;
          enable_registration = false;
          database = {
            name = "psycopg2";
            allow_unsafe_locale = true;
            args = {
              user = "matrix-synapse";
              database = "matrix-synapse";
              host = "/run/postgresql";
            };
          };
        };
        extraConfigFiles = [
          config.age.secrets.matrix-password.path
        ];
      };
      mautrix-whatsapp = {
        enable = true;
        package = pkgs-unstable.mautrix-whatsapp;
        environmentFile = config.age.secrets.whatsapp_secrets.path;
        settings = {
          appservice = {
            as_token = "$MAUTRIX_WHATSAPP_APPSERVICE_AS_TOKEN";
            hs_token = "$MAUTRIX_WHATSAPP_APPSERVICE_HS_TOKEN";
          };
          database = {
            type = "postgres";
            uri = "postgres://mautrix-whatsapp@/mautrix-whatsapp?host=/run/postgresql";
          };
          homeserver = {
            address = "http://127.0.0.1:8008";
            domain = config.xanterella.matrix-server.domain;
          };
          bridge = {
            permissions = {
              "@xeravus:${config.xanterella.matrix-server.domain}" = "admin";
            };
          };
        };
      };
      mautrix-discord = {
        enable = true;
        environmentFile = config.age.secrets.discord_secrets.path;
        settings = {
          appservice = {
            as_token = "$MAUTRIX_DISCORD_APPSERVICE_AS_TOKEN";
            hs_token = "$MAUTRIX_DISCORD_APPSERVICE_HS_TOKEN";
            database = {
              type = "postgres";
              uri = "postgres://mautrix-discord@/mautrix-discord?host=/run/postgresql";
            };
          };
          homeserver = {
            address = "http://127.0.0.1:8008";
            domain = config.xanterella.matrix-server.domain;
          };
          bridge = {
            permissions = {
              "@xeravus:${config.xanterella.matrix-server.domain}" = "admin";
            };
          };
        };
      };
      tailscale = {
        permitCertUid = "caddy";
      };
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
    nixpkgs = {
      config = {
        permittedInsecurePackages = [
          "olm-3.2.16"
        ];
      };
    };
  };
}
