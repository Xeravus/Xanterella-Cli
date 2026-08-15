{
  config,
  inputs,
  lib,
  pkgs,
  ...
}: {
  config = lib.mkIf config.xanterella.monitoring.enable {
    networking.firewall.allowedTCPPorts = [
      config.services.grafana.settings.server.http_port
    ];
    services.caddy.enable = true;
    services.caddy.globalConfig = ''
          servers {
            metrics
          }
        '';
    services.caddy.virtualHosts."https://${config.xanterella.monitoring.domain}".extraConfig = ''
              handle /grafana* {
                       reverse_proxy ${config.services.grafana.settings.server.http_addr}:${toString config.services.grafana.settings.server.http_port}
                }
            '';
    services.grafana.enable = true;
    services.grafana.provision.dashboards.settings.providers = [
      {
        name = "GitHub Dashboard";
        options.path = "${inputs.xanterella-etc}";
      }
    ];
    services.grafana.provision.datasources.settings.datasources = [
      {
        access = "proxy";
        isDefault = true;
        name = "Prometheus";
        type = "prometheus";
        url = "http://127.0.0.1:${toString config.services.prometheus.port}";
      }
    ];
    services.grafana.provision.enable = true;
    services.grafana.settings.server.domain = config.xanterella.monitoring.domain;
    services.grafana.settings.server.http_addr = "127.0.0.1";
    services.grafana.settings.server.http_port = 9000;
    services.grafana.settings.server.root_url = "%(protocol)s://%(domain)s/grafana/";
    services.grafana.settings.server.serve_from_sub_path = true;
    services.prometheus.enable = true;
    services.prometheus.exporters.node.enable = true;
    services.prometheus.exporters.node.enabledCollectors = [
      "systemd"
    ];
    services.prometheus.exporters.node.listenAddress = "127.0.0.1";
    services.prometheus.exporters.node.port = 9100;
    services.prometheus.listenAddress = "127.0.0.1";
    services.prometheus.port = 9090;
    services.prometheus.retentionTime = "15d";
    services.prometheus.scrapeConfigs = [
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
    services.tailscale.permitCertUid = "caddy";
    systemd.services.grafana.environment.GF_DASHBOARDS_DEFAULT_HOME_DASHBOARD_PATH = "${inputs.xanterella-etc}/grafana/monitoring.json";
    users.users.caddy.extraGroups = [
      "tailscale"
    ];
  };
  options.xanterella.monitoring.domain = lib.mkOption {
    default = "xanterella.de/monitoring";
    type = lib.types.str;
  };
  options.xanterella.monitoring.enable = lib.mkEnableOption "Aktiviert Monitoring";
}