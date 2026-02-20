# Home Manager module for i3-cycles
#
# Usage in your home.nix:
#
#   imports = [ inputs.i3-cycles.homeManagerModules.default ];
#
#   programs.i3-cycles = {
#     enable = true;
#     logFormat = "plain";  # or "json"
#   };
#
# The daemon will start automatically on graphical session login
# and will be available at ~/.local/share/i3-cycle/daemon.sock

{
  config,
  lib,
  pkgs,
  ...
}:

with lib;

let
  cfg = config.programs.i3-cycles;
in
{
  options.programs.i3-cycles = {
    enable = mkEnableOption "i3-cycles daemon - manages workspace cycles";

    package = mkOption {
      type = types.package;
      default = pkgs.i3-cycles or (throw "i3-cycles package not available");
      defaultText = literalExpression "pkgs.i3-cycles";
      description = ''
        The i3-cycles package to use.
      '';
    };

    logFormat = mkOption {
      type = types.enum [
        "plain"
        "json"
      ];
      default = "plain";
      description = ''
        The log output format for the daemon.
        - "plain": Human-readable plain text format
        - "json": Structured JSON format
      '';
    };

    dataDir = mkOption {
      type = types.path;
      default = "${config.home.homeDirectory}/.local/share/i3-cycle";
      defaultText = literalExpression ''"''${config.home.homeDirectory}/.local/share/i3-cycle"'';
      description = ''
        Directory for daemon socket and state files.
      '';
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];

    # Ensure the data directory exists
    home.file."${cfg.dataDir}/.keep" = {
      text = "";
      executable = false;
    };

    # Systemd user service for the i3-cycle daemon
    systemd.user.services.i3-cycle-daemon = {
      Unit = {
        Description = "i3-cycle daemon - manages workspace cycles";
        After = [ "graphical-session-pre.target" ];
        PartOf = [ "graphical-session.target" ];
      };

      Service = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/i3-cycle-daemon --log-format=${cfg.logFormat}";
        Restart = "on-failure";
        RestartSec = 5;
        Environment = "XDG_RUNTIME_DIR=%t";
      };

      Install = {
        WantedBy = [ "graphical-session.target" ];
      };
    };
  };
}
