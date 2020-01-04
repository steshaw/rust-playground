let
  pinnedPkgs = import ./pkgs.nix;
in { pkgs ? pinnedPkgs }:
let
  moz_overlay = import (
    builtins.fetchTarball
      "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz"
  );
  pkgs_overlay = pkgs {
    overlays = [
      moz_overlay
    ];
  };
  #rustNightlyChannel = (nixpkgs.rustChannelOf { date = "2019-01-26"; channel = "nightly"; }).rust;
  rustStableChannel = pkgs_overlay.latest.rustChannels.stable.rust.override {
    extensions = [
      "rust-src"
      "rls-preview"
      "clippy-preview"
      "rustfmt-preview"
    ];
  };
in with pkgs_overlay;
stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs = [
    rustStableChannel
    rls
    rustup

    cacert
    clang
    clang-tools
    gnumake
  ];
}
