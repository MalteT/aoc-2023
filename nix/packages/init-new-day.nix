{
  coreutils,
  git,
  writeShellApplication,
  rustToolchain,
  ...
}:
writeShellApplication {
  name = "init-new-day";
  runtimeInputs = [
    coreutils
    git
    rustToolchain
  ];
  text = builtins.readFile ../../scripts/init-new-day.sh;
}
