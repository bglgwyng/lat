{
  description = "cat for LLMs";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    lat-json-viewer.url = "github:bglgwyng/lat-json-viewer";
    lat-js-viewer.url = "github:bglgwyng/lat-js-viewer";
    lat-plaintext-viewer.url = "github:bglgwyng/lat-plaintext-viewer";
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
          packages.default = pkgs.writeShellScriptBin "lat" ''
            file="$1"
            shift
            case "$file" in
              *.json)
                exec ${inputs'.lat-json-viewer.packages.default}/bin/lat-json-viewer "$file" "$@"
                ;;
              *.js|*.ts|*.jsx|*.tsx|*.cjs|*.mjs)
                exec ${inputs'.lat-js-viewer.packages.default}/bin/lat-js-viewer "$file" "$@"
                ;;
              *)
                exec ${inputs'.lat-plaintext-viewer.packages.default}/bin/lat-plaintext-viewer "$file" "$@"
                ;;
            esac
          '';
          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
