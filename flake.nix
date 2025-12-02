{
  description = "Advent of Code 2025";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: 
  let
    pkgs = (import nixpkgs { inherit system; });
    day = "1-1";
    version = "2025.12." + day;
  in {
    packages = {
      day1-1 = pkgs.callPackage ./. { 
        inherit version day; 
      };
    };
    apps = {
      day1-1 = {
        type = "app";
        program = "${pkgs.writeShellScriptBin "day1-1" ''
          #!/usr/bin/env bash
          cat ${./inputs/day1.txt} | ${self.packages.${system}.day1-1}/bin/day1-1
        ''}/bin/day1-1";
      };
    };

    packages.default = self.packages.${system}."day${day}";
    apps.default = self.apps.${system}."day${day}";

    devShells.default = pkgs.mkShell {
      inputsFrom = [
        self.packages.${system}.default
      ];
      nativeBuildInputs = with pkgs; [
        rust-analyzer
        rustfmt
      ];
    };
  });
}
