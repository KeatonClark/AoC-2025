{
  rustPlatform,
  version,
}:
rustPlatform.buildRustPackage {
  inherit version;
  pname = "AdventOfCode";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  doCheck = true;
}
