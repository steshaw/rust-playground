#
# See https://nixos.wiki/wiki/FAQ/Pinning_Nixpkgs.
#
import (
  builtins.fetchGit {
    # Descriptive name to make the store path easier to identify.
    name = "nixpkgs-20191207-121557";
    url = https://github.com/nixos/nixpkgs/;
    # `git ls-remote https://github.com/NixOS/nixpkgs-channels nixpkgs-unstable`
    rev = "fb1bc1b891f5c95b98a333e46b043018ff50d254";
  }
)
