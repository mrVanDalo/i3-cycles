{
  description = "Description for the project";

  inputs = {
    devshell.url = "github:numtide/devshell";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./nix/formatter.nix
        ./nix/devshells.nix
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.

          # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;
          packages = {
            default = pkgs.callPackage ./. { };
            i3-cycles = pkgs.callPackage ./. { };
          };

          checks = {
            i3-cycles = pkgs.callPackage ./test.nix {
              inherit (pkgs) testers;
              inherit (self'.packages) i3-cycles;
            };
            daemon-integration = pkgs.callPackage ./tests/nixos/daemon_integration_test.nix {
              inherit (pkgs) testers;
              inherit (self'.packages) i3-cycles;
            };
          };
        };
      flake = {
        # Overlay that can be imported in other flakes
        overlays = {
          default = final: prev: {
            i3-cycles = final.callPackage ./. { };
          };
          i3-cycles = final: prev: {
            i3-cycles = final.callPackage ./. { };
          };
        };

        # Home Manager module export
        hmModules = {
          default = import ./nix/hm-module.nix;
          i3-cycles = import ./nix/hm-module.nix;
        };
      };
    };
}
