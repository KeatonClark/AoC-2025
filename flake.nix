{
  description = "Advent of Code 2025";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: 
  let
    pkgs = (import nixpkgs { inherit system; });
    day = "1";
    version = "2025.12." + day;
  in {
    packages = {
      day1 = pkgs.callPackage ./. { day = "1"; inherit version; };
    };

    packages.default = self.packages.${system}.day1;

    devShells.default = pkgs.mkShell {
      inputsFrom = [
        self.packages.${system}.default
      ];
      nativeBuildInputs = with pkgs; [
        rust-analyzer
      ];
    };
  });
}
