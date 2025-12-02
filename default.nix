{
  rustPlatform,
  day,
  version,
}:
rustPlatform.buildRustPackage {
  inherit version;
  pname = "AdventOfCode";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  bin = "day" + day;
  doCheck = true;
}
