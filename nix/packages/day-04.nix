{
  rustPlatform,
  lib,
  ...
}: let
  package = "day-04";
  src = with lib.sources;
    cleanSourceWith {
      name = "${package}-src";
      src = sourceByRegex ../.. [
        "^.*Cargo.(toml|lock)$"
        "^(aoc-utils|day-.*)(/src.*)?$"
        "^inputs.*$"
      ];
      filter = cleanSourceFilter;
    };
  pkgConfig = lib.trivial.importTOML ../../${package}/Cargo.toml;
in
  rustPlatform.buildRustPackage {
    pname = pkgConfig.package.name;
    version = pkgConfig.package.version;

    inherit src;

    cargoBuildFlags = [
      "--package ${pkgConfig.package.name}"
    ];

    cargoDeps = {
      lockFile = "../../Cargo.lock";
    };

    cargoLock = {
      lockFile = ../../Cargo.lock;
    };

    meta = with lib; {
      license = licenses.mit;
    };
  }
