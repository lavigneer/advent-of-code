{
  description = "Advent of Code";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        # Nix fmt
        formatter = nixpkgs.legacyPackages.x86_64-linux.nixfmt-rfc-style;

        # shell in nix develop
        devShells.default = nixpkgs.legacyPackages.${system}.mkShell {
          packages = [
            pkgs.gnumake
            pkgs.wget
            pkgs.ocaml
            pkgs.opam
            pkgs.dune_3
            pkgs.kind
          ];
        };
      }
    );
}
