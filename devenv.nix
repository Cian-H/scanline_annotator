{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs; [
    act
    cargo-bump
    git
    git-lfs
    pre-commit
    ruff
  ];

  env.NIX_LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
    stdenv.cc.cc
    zlib
  ]);
  env.NIX_LD = lib.fileContents "${pkgs.stdenv.cc}/nix-support/dynamic-linker";

  enterShell = ''
    unset PYTHONPATH
  '';

  languages = {
    python = {
      version = "3.14";
      enable = true;
      uv = {
        enable = true;
        sync.enable = true;
      };
    };
  };
  languages.rust.enable = true;
}
