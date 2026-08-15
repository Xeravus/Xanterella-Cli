#left:
{
  config,
  inputs,
  lib,
  pkgs,
  ...
}: {
  config = lib.mkIf config.xanterella.monitoring.enable {
    networking = {firewall = {allowedTCPPorts = [config.services.grafana.settings.server.http_port];};};
    services = {
      caddy = {
        enable = true;
        globalConfig = ''servers {            metrics          }        '';
        virtualHosts = {"https://${config.xanterella.monitoring.domain}" = {extraConfig = ''handle /grafana* {                       reverse_proxy ${config.services.grafana.settings.server.http_addr}:${toString config.services.grafana.settings.server.http_port}                }            '';};};
      };
      grafana = {
        enable = true;
        provision = {
          dashboards = {
            settings = {
              providers = [
                {
                  name = "GitHub Dashboard";
                  options = {path = "${inputs.xanterella-etc}";};
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
            enabledCollectors = ["hwmon" "systemd" "tcpstat"];
            listenAddress = "127.0.0.1";
            port = 9100;
          };
          process = {
            enable = true;
            listenAddress = "127.0.0.1";
            port = 9101;
            settings = {
              process_names = [
                {
                  cmdline = [".*atticd.*"];
                  name = "Attic";
                }
                {
                  cmdline = [".*audiobookshelf.*"];
                  name = "Audiobookshelf";
                }
                {
                  cmdline = [".*caddy.*"];
                  name = "Caddy";
                }
                {
                  cmdline = [".*github-runner.*"];
                  name = "GitHub-Runner";
                }
                {
                  cmdline = [".*grafana.*"];
                  name = "Grafana";
                }
                {
                  cmdline = [".*mautrix-discord.*"];
                  name = "Matrix Discord";
                }
                {
                  cmdline = [".*synapse.*"];
                  name = "Matrix Synapse";
                }
                {
                  cmdline = [".*mautrix-whatsapp.*"];
                  name = "Matrix Whatsapp";
                }
                {
                  cmdline = [".*netbird.*"];
                  name = "Netbird";
                }
                {
                  cmdline = [".*tailscaled.*"];
                  name = "Tailscale";
                }
                {
                  cmdline = [".*vaultwarden.*"];
                  name = "Vaultwarden";
                }
                {
                  cmdline = [".*vikunja.*"];
                  name = "Vikunja";
                }
              ];
            };
          };
        };
        listenAddress = "127.0.0.1";
        port = 9090;
        retentionTime = "15d";
        scrapeConfigs = [
          {
            job_name = "caddy";
            scrape_interval = "15s";
            static_configs = [{targets = ["127.0.0.1:2019"];}];
          }
          {
            job_name = "node_exporter";
            scrape_interval = "15s";
            static_configs = [{targets = ["127.0.0.1:${toString config.services.prometheus.exporters.node.port}"];}];
          }
          {
            job_name = "process_exporter";
            scrape_interval = "30s";
            static_configs = [{targets = ["127.0.0.1:${toString config.services.prometheus.exporters.process.port}"];}];
          }
        ];
      };
      tailscale = {permitCertUid = "caddy";};
    };
    systemd = {services = {grafana = {environment = {GF_DASHBOARDS_DEFAULT_HOME_DASHBOARD_PATH = "${inputs.xanterella-etc}/grafana/monitoring.json";};};};};
    users = {users = {caddy = {extraGroups = ["tailscale"];};};};
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
