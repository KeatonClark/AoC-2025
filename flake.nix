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
      day2-1 = (mkDay "2-1" ./inputs/day2.txt);
      day2-2 = (mkDay "2-2" ./inputs/day2.txt);
      day3-1 = (mkDay "3-1" ./inputs/day3.txt);
      day3-2 = (mkDay "3-2" ./inputs/day3.txt);
      day4-1 = (mkDay "4-1" ./inputs/day4.txt);
      day4-2 = (mkDay "4-2" ./inputs/day4.txt);
      day5-1 = (mkDay "5-1" ./inputs/day5.txt);
      day5-2 = (mkDay "5-2" ./inputs/day5.txt);
      day6-1 = (mkDay "6-1" ./inputs/day6.txt);
      day6-2 = (mkDay "6-2" ./inputs/day6.txt);
      day7-1 = (mkDay "7-1" ./inputs/day7.txt);
      day7-2 = (mkDay "7-2" ./inputs/day7.txt);
      day8-1 = (mkDay "8-1" ./inputs/day8.txt);
      day8-2 = (mkDay "8-2" ./inputs/day8.txt);
      day9-1 = (mkDay "9-1" ./inputs/day9.txt);
      day9-2 = (mkDay "9-2" ./inputs/day9.txt);
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
