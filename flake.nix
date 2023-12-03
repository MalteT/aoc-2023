{
  inputs.devshell.url = "github:numtide/devshell";
  inputs.flake-parts.url = "github:hercules-ci/flake-parts";
  inputs.treefmt-nix.url = "github:numtide/treefmt-nix";
  inputs.pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    self,
    flake-parts,
    devshell,
    treefmt-nix,
    pre-commit-hooks-nix,
    fenix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs self;} {
      imports = [
        devshell.flakeModule
        treefmt-nix.flakeModule
        pre-commit-hooks-nix.flakeModule
      ];
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];
      flake.hydraJobs.packages.x86_64-linux = self.packages.x86_64-linux;
      flake.hydraJobs.devShells.x86_64-linux = self.devShells.x86_64-linux;
      flake.hydraJobs.checks.x86_64-linux = self.checks.x86_64-linux;
      perSystem = {
        self',
        pkgs,
        lib,
        config,
        system,
        ...
      }: let
        rustToolchain = with fenix.packages.${system};
          combine [
            (complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            targets.wasm32-unknown-unknown.latest.rust-std
            rust-analyzer
          ];

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in {
        pre-commit.check.enable = false;
        pre-commit.settings.hooks.markdownlint.enable = true;
        pre-commit.settings.hooks.nil.enable = true;
        pre-commit.settings.hooks.format = {
          enable = true;
          entry = "${self'.formatter}/bin/treefmt";
          pass_filenames = false;
        };
        pre-commit.settings.hooks.my-clippy = {
          enable = true;
          name = "clippy";
          description = "Lint Rust code.";
          entry = "${rustToolchain}/bin/cargo-clippy clippy --offline -- -D warnings";
          files = "\\.rs$";
          pass_filenames = false;
        };
        pre-commit.settings.hooks.my-cargo-check = {
          enable = true;
          description = "check the cargo package for errors.";
          entry = "${rustToolchain}/bin/cargo check --offline";
          files = "\\.rs$";
          pass_filenames = false;
        };
        # Packages
        packages = let
          createPackage = file: _type: {
            name = lib.strings.removeSuffix ".nix" (builtins.baseNameOf file);
            value = pkgs.callPackage ./nix/packages/${file} {inherit rustPlatform rustToolchain;};
          };
        in
          lib.attrsets.mapAttrs' createPackage (builtins.readDir ./nix/packages);
        # Shell
        devShells.default = pkgs.mkShell {
          name = "dev";
          shellHook = ''
            ${config.pre-commit.installationScript}
            echo 1>&2 "Welcome to the development shell!"
          '';
          nativeBuildInputs = [
            config.treefmt.package
            rustToolchain
            pkgs.nil
            pkgs.hyperfine
            pkgs.cargo-flamegraph
            self'.packages.init-new-day
            self'.packages.perf
          ];
          RUST_LOG = "trace";
        };
        devShells.pre-commit = config.pre-commit.devShell;
        # Formatter
        treefmt.projectRootFile = "flake.nix";
        treefmt.programs = {
          rustfmt.enable = true;
          alejandra.enable = true;
        };
        treefmt.flakeFormatter = true;
      };
    };
}
