{
  config,
  lib,
  pkgs,
  pkgs-unstable,
  ...
}: {
  config = lib.mkIf config.xanterella.matrix-server.enable {
    age.secrets.discord_secrets.file = ./../agenix/mautrix_disord.env.age;
    age.secrets.discord_secrets.group = "mautrix-discord";
    age.secrets.discord_secrets.owner = "mautrix-discord";
    age.secrets.matrix-password.file = ./../agenix/matrix.yaml.age;
    age.secrets.matrix-password.group = "matrix-synapse";
    age.secrets.matrix-password.owner = "matrix-synapse";
    age.secrets.whatsapp_secrets.file = ./../agenix/mautrix_whatsapp.env.age;
    age.secrets.whatsapp_secrets.group = "mautrix-whatsapp";
    age.secrets.whatsapp_secrets.owner = "mautrix-whatsapp";
    nixpkgs.config.permittedInsecurePackages = [
      "olm-3.2.16"
    ];
    services.caddy.enable = true;
    services.caddy.virtualHosts."https://${config.xanterella.matrix-server.domain}".extraConfig = ''
              handle /_matrix* {
              reverse_proxy 127.0.0.1:8008
              }
              handle /_synapse/client* {
              reverse_proxy 127.0.0.1:8008
              }
            '';
    services.matrix-synapse.enable = true;
    services.matrix-synapse.extraConfigFiles = [
      config.age.secrets.matrix-password.path
    ];
    services.matrix-synapse.settings.database.allow_unsafe_locale = true;
    services.matrix-synapse.settings.database.args.database = "matrix-synapse";
    services.matrix-synapse.settings.database.args.host = "/run/postgresql";
    services.matrix-synapse.settings.database.args.user = "matrix-synapse";
    services.matrix-synapse.settings.database.name = "psycopg2";
    services.matrix-synapse.settings.enable_registration = false;
    services.matrix-synapse.settings.server_name = config.xanterella.matrix-server.domain;
    services.mautrix-discord.enable = true;
    services.mautrix-discord.environmentFile = config.age.secrets.discord_secrets.path;
    services.mautrix-discord.settings.appservice.as_token = "$MAUTRIX_DISCORD_APPSERVICE_AS_TOKEN";
    services.mautrix-discord.settings.appservice.database.type = "postgres";
    services.mautrix-discord.settings.appservice.database.uri = "postgres://mautrix-discord@/mautrix-discord?host=/run/postgresql";
    services.mautrix-discord.settings.appservice.hs_token = "$MAUTRIX_DISCORD_APPSERVICE_HS_TOKEN";
    services.mautrix-discord.settings.bridge.permissions."@xeravus:${config.xanterella.matrix-server.domain}" = "admin";
    services.mautrix-discord.settings.homeserver.address = "http://127.0.0.1:8008";
    services.mautrix-discord.settings.homeserver.domain = config.xanterella.matrix-server.domain;
    services.mautrix-whatsapp.enable = true;
    services.mautrix-whatsapp.environmentFile = config.age.secrets.whatsapp_secrets.path;
    services.mautrix-whatsapp.package = pkgs-unstable.mautrix-whatsapp;
    services.mautrix-whatsapp.settings.appservice.as_token = "$MAUTRIX_WHATSAPP_APPSERVICE_AS_TOKEN";
    services.mautrix-whatsapp.settings.appservice.hs_token = "$MAUTRIX_WHATSAPP_APPSERVICE_HS_TOKEN";
    services.mautrix-whatsapp.settings.bridge.permissions."@xeravus:${config.xanterella.matrix-server.domain}" = "admin";
    services.mautrix-whatsapp.settings.database.type = "postgres";
    services.mautrix-whatsapp.settings.database.uri = "postgres://mautrix-whatsapp@/mautrix-whatsapp?host=/run/postgresql";
    services.mautrix-whatsapp.settings.homeserver.address = "http://127.0.0.1:8008";
    services.mautrix-whatsapp.settings.homeserver.domain = config.xanterella.matrix-server.domain;
    services.postgresql.enable = true;
    services.postgresql.ensureDatabases = [
      "matrix-synapse"
      "mautrix-whatsapp"
      "mautrix-discord"
    ];
    services.postgresql.ensureUsers = [
      {
        ensureDBOwnership = true;
        name = "matrix-synapse";
      }
      {
        ensureDBOwnership = true;
        name = "mautrix-whatsapp";
      }
      {
        ensureDBOwnership = true;
        name = "mautrix-discord";
      }
    ];
    services.tailscale.permitCertUid = "caddy";
    users.users.caddy.extraGroups = [
      "tailscale"
    ];
  };
  options.xanterella.matrix-server.domain = lib.mkOption {
    default = "xanterella.de/matrix";
    type = lib.types.str;
  };
  options.xanterella.matrix-server.enable = lib.mkEnableOption "Aktiviert Matrix Pipeline";
}