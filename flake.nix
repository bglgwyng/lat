{
  description = "cat for LLMs";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
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
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              (import inputs.rust-overlay)
            ];
            config = { };
          };
          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "lat";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              (pkgs.rust-bin.stable."1.91.1".default.override {
                extensions = [ "rust-src" ];
              })
            ];
          };
          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
