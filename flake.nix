{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        run-roxy-loader = pkgs.writeShellApplication {
          name = "run-roxy-loader";
          runtimeInputs = [
            toolchain
            pkgs.qemu
          ];
          text = ''
            cargo run -p runner -- "$@"
          '';
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
            pkgs.qemu
          ];
        };

        apps.default = {
          type = "app";
          program = "${run-roxy-loader}/bin/run-roxy-loader";
        };
      }
    );
}
