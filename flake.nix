{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in rec
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            name = "mega-utils";
            src = self;
            useFetchCargoVendor = true;
            cargoHash = "sha256-bGOdzQQCX8HdlRp64NnNf60iZUBuARKGzHyugO8YFxg=";
            buildType = "debug";
            dontStrip = true;
          };
        };
        devShells.default = packages.default.overrideAttrs (final: old: {
          nativeBuildInputs = old.nativeBuildInputs ++ (with pkgs; [ clippy rustfmt ]);
        });
      }
    );
}
