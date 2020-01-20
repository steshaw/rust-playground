let pinnedPkgs = import ./pkgs.nix;
in { pkgs ? pinnedPkgs }:
let
  #
  # Using Mozilla Rust overlay:
  #   https://github.com/mozilla/nixpkgs-mozilla.
  #
  mozOverlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  mozPkgs = pkgs { overlays = [ mozOverlay ]; };

  rustStableExtensions = [
    "clippy-preview"
    #"lldb-preview"
    "llvm-tools-preview"
    #"miri-preview"
    "rls-preview"
    "rust-analysis"
    "rust-src"
    "rust-std"
    #"rustc-dev"
    "rustfmt-preview"
  ];

  rustNightlyExtensions = [
    #"clippy-preview"
    #"lldb-preview"
    "llvm-tools-preview"
    "miri-preview"
    "rls-preview"
    "rust-analysis"
    "rust-src"
    "rust-std"
    "rustc-dev"
    "rustfmt-preview"
  ];

  enableNightly = true;
  rust = if enableNightly then
    let
      enableLatest = false;
      rustChannel = if enableLatest then
        mozPkgs.latest.rustChannels.nightly
      else
        mozPkgs.rustChannelOf {
          date = "2019-12-15";
          channel = "nightly";
        };
    in rustChannel.rust.override { extensions = rustNightlyExtensions; }
  else
    mozPkgs.latest.rustChannels.stable.rust.override {
      extensions = rustStableExtensions;
    };

in with mozPkgs;
stdenv.mkDerivation {
  name = "learning-rust-shell";
  buildInputs = [
    rust

    cacert
    cargo-edit
    gtk3
    ncurses
    watchexec

    clang
    clang-tools
    gnumake
  ];

  RUST_BACKTRACE = 1;
}
