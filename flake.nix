{
  description = "Advent of Code 2025";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: 
  let
    pkgs = (import nixpkgs { inherit system; });
    version = "2025.12.1";
    mkDay = (day: input: {
      type = "app";
      program = "${pkgs.writeShellScriptBin "day${day}" ''
        #!/usr/bin/env bash
        cat ${input} | ${self.packages.${system}.default}/bin/day${day}
      ''}/bin/day${day}";
    });
  in {
    apps = {
      day1-1 = (mkDay "1-1" ./inputs/day1.txt);
      day1-2 = (mkDay "1-2" ./inputs/day1.txt);
    };

    packages.default = pkgs.callPackage ./. { inherit version; };

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
