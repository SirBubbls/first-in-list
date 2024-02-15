{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        name = "trajectory-trace";

        rustBuild = naersk-lib.buildPackage {
          src = ./trajectory-trace;
          buildInputs = with pkgs; [ cargo rustc postgresql ];
        };
        dockerImage = pkgs.dockerTools.buildImage {
          name = name;
          tag = rustBuild.version;
          copyToRoot = [ pkgs.cacert ];
          config = {
            Entrypoint = [ "${rustBuild}/bin/${name}" ];
          };
        };
      in
      {
        defaultPackage = rustBuild;
        packages.image = dockerImage;

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy postgresql diesel-cli kubernetes-helm ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          shellHook = ''
            export DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres"
            export RUST_LOG=info
            export CONFIG_PATH="./config.toml"
          '';
        };
      });
}
