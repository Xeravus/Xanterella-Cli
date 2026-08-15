{
  config,
  pkgs,
  inputs,
  lib,
  ...
}: {
  options = {
    xanterella = {
      monitoring = {
        enable = lib.mkEnableOption "Aktiviert Monitoring";
        domain = lib.mkOption {
          type = lib.types.str;
          default = "xanterella.de/monitoring";
        };
      };
    };
  };
  config = lib.mkIf config.xanterella.monitoring.enable {
    services = {
      prometheus = {
        enable = true;
        port = 9090;
        listenAddress = "127.0.0.1";
        retentionTime = "15d";
        exporters = {
          node = {
            enable = true;
            enabledCollectors = [
              "systemd"
              "hwmon"
              "tcpstat"
            ];
            port = 9100;
            listenAddress = "127.0.0.1";
          };
          process = {
            enable = true;
            port = 9101;
            listenAddress = "127.0.0.1";
            settings = {
              process_names = [
                {
                  name = "Netbird";
                  cmdline = [
                    ".*netbird.*"
                  ];
                }
                {
                  name = "Tailscale";
                  cmdline = [
                    ".*tailscaled.*"
                  ];
                }
                {
                  name = "Caddy";
                  cmdline = [
                    ".*caddy.*"
                  ];
                }
                {
                  name = "Grafana";
                  cmdline = [
                    ".*grafana.*"
                  ];
                }
                {
                  name = "GitHub-Runner";
                  cmdline = [
                    ".*github-runner.*"
                  ];
                }
                {
                  name = "Vikunja";
                  cmdline = [
                    ".*vikunja.*"
                  ];
                }
                {
                  name = "Vaultwarden";
                  cmdline = [
                    ".*vaultwarden.*"
                  ];
                }
                {
                  name = "Audiobookshelf";
                  cmdline = [
                    ".*audiobookshelf.*"
                  ];
                }
                {
                  name = "Matrix Synapse";
                  cmdline = [
                    ".*synapse.*"
                  ];
                }
                {
                  name = "Matrix Discord";
                  cmdline = [
                    ".*mautrix-discord.*"
                  ];
                }
                {
                  name = "Matrix Whatsapp";
                  cmdline = [
                    ".*mautrix-whatsapp.*"
                  ];
                }
                {
                  name = "Attic";
                  cmdline = [
                    ".*atticd.*"
                  ];
                }
              ];
            };
          };
        };
        scrapeConfigs = [
          {
            job_name = "node_exporter";
            scrape_interval = "15s";
            static_configs = [
              {
                targets = [
                  "127.0.0.1:${toString config.services.prometheus.exporters.node.port}"
                ];
              }
            ];
          }
          {
            job_name = "process_exporter";
            scrape_interval = "30s";
            static_configs = [
              {
                targets = [
                  "127.0.0.1:${toString config.services.prometheus.exporters.process.port}"
                ];
              }
            ];
          }
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
        ];
      };
      grafana = {
        enable = true;
        settings = {
          server = {
            http_addr = "127.0.0.1";
            http_port = 9000;
            domain = config.xanterella.monitoring.domain;
            root_url = "%(protocol)s://%(domain)s/grafana/";
            serve_from_sub_path = true;
          };
        };
        provision = {
          enable = true;
          datasources = {
            settings = {
              datasources = [
                {
                  name = "Prometheus";
                  type = "prometheus";
                  access = "proxy";
                  url = "http://127.0.0.1:${toString config.services.prometheus.port}";
                  isDefault = true;
                }
              ];
            };
          };
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
        };
      };
      tailscale = {
        permitCertUid = "caddy";
      };
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
    };
    networking = {
      firewall = {
        allowedTCPPorts = [
          config.services.grafana.settings.server.http_port
        ];
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
    systemd = {
      services = {
        grafana = {
          environment = {
            GF_DASHBOARDS_DEFAULT_HOME_DASHBOARD_PATH = "${inputs.xanterella-etc}/grafana/monitoring.json";
          };
        };
      };
    };
  };
}