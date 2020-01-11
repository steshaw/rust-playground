let
  pinnedPkgs = import ./pkgs.nix;
in
{ pkgs ? pinnedPkgs }:
  let
    mozOverlay = import (
      builtins.fetchTarball
        "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz"
    );
    mozPkgs = pkgs {
      overlays = [
        mozOverlay
      ];
    };

    useNightly = true;

    rust = if useNightly then
      let
        rustChannel = mozPkgs.rustChannelOf {
          date = "2020-01-11";
          channel = "nightly";
        };
      in
        rustChannel.rust
    else
      mozPkgs.latest.rustChannels.stable.rust.override {
        extensions = [
          "clippy-preview"
          "rls-preview"
          "rust-src"
          "rustfmt-preview"
        ];
      };
  in
    with mozPkgs;
    stdenv.mkDerivation {
      name = "moz_overlay_shell";
      buildInputs = [
        rust

        cacert
        watchexec

        clang
        clang-tools
        gnumake
      ];
    }
