{
  writeShellApplication,
  hyperfine,
  nixFlakes,
  coreutils,
  ...
}:
writeShellApplication {
  name = "perf";
  runtimeInputs = [
    hyperfine
    nixFlakes
    coreutils
  ];
  text = builtins.readFile ../../scripts/perf.sh;
}
