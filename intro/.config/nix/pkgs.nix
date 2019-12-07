#
# See https://nixos.wiki/wiki/FAQ/Pinning_Nixpkgs.
#
import (
  builtins.fetchGit {
    # Descriptive name to make the store path easier to identify.
    name = "nixpkgs-20191207-121557";
    url = https://github.com/nixos/nixpkgs/;
    # `git ls-remote https://github.com/NixOS/nixpkgs-channels nixpkgs-unstable`
    rev = "cc6cf0a96a627e678ffc996a8f9d1416200d6c81";
  }
)
