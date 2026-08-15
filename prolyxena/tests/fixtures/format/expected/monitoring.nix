{
  config,
  inputs,
  lib,
  pkgs,
  ...
}: {
  config = lib.mkIf config.xanterella.monitoring.enable {
    networking = {
      firewall = {
        allowedTCPPorts = [
          config.services.grafana.settings.server.http_port
        ];
      };
    };
    services = {
      caddy = {
        enable = true;
        globalConfig = ''
          servers {
            metrics
          }
        '';
        virtualHosts = {
          "https://${config.xanterella.monitoring.domain}" = {
            extraConfig = ''
              handle /grafana* {
                       reverse_proxy ${config.services.grafana.settings.server.http_addr}:${toString config.services.grafana.settings.server.http_port}
                }
            '';
          };
        };
      };
      grafana = {
        enable = true;
        provision = {
          dashboards = {
            settings = {
              providers = [
                {
                  name = "GitHub Dashboard";
                  options = {
                    path = "${inputs.xanterella-etc}";
                  };
                }
              ];
            };
          };
          datasources = {
            settings = {
              datasources = [
                {
                  access = "proxy";
                  isDefault = true;
                  name = "Prometheus";
                  type = "prometheus";
                  url = "http://127.0.0.1:${toString config.services.prometheus.port}";
                }
              ];
            };
          };
          enable = true;
        };
        settings = {
          server = {
            domain = config.xanterella.monitoring.domain;
            http_addr = "127.0.0.1";
            http_port = 9000;
            root_url = "%(protocol)s://%(domain)s/grafana/";
            serve_from_sub_path = true;
          };
        };
      };
      prometheus = {
        enable = true;
        exporters = {
          node = {
            enable = true;
            enabledCollectors = [
              "systemd"
            ];
            listenAddress = "127.0.0.1";
            port = 9100;
          };
        };
        listenAddress = "127.0.0.1";
        port = 9090;
        retentionTime = "15d";
        scrapeConfigs = [
          {
            job_name = "caddy";
            scrape_interval = "15s";
            static_configs = [
              {
                targets = [
                  "127.0.0.1:2019"
                ];
              }
            ];
          }
          {
            job_name = "nixos-laptop";
            scrape_interval = "15s";
            static_configs = [
              {
                targets = [
                  "127.0.0.1:${toString config.services.prometheus.exporters.node.port}"
                ];
              }
            ];
          }
        ];
      };
      tailscale = {
        permitCertUid = "caddy";
      };
    };
    systemd = {
      services = {
        grafana = {
          environment = {
            GF_DASHBOARDS_DEFAULT_HOME_DASHBOARD_PATH = "${inputs.xanterella-etc}/grafana/monitoring.json";
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
  };
  options = {
    xanterella = {
      monitoring = {
        domain = lib.mkOption {
          default = "xanterella.de/monitoring";
          type = lib.types.str;
        };
        enable = lib.mkEnableOption "Aktiviert Monitoring";
      };
    };
  };
}