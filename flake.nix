{
  description = "A nostalgic infinite installer simulator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "install-nothing";
            version = "0.4.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            meta = with pkgs.lib; {
              description = "A nostalgic infinite installer simulator";
              longDescription = ''
                A terminal application that simulates installing things.
                It doesn't actually install anything.
              '';
              homepage = "https://github.com/buyukakyuz/install-nothing";
              license = licenses.mit;
              maintainers = [ ];
              platforms = platforms.unix;
            };
          };

          install-nothing = self.packages.${system}.default;
        };

        apps = {
          default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/install-nothing";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
          ];
        };
      }
    );
}
