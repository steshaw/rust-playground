#
# See https://nixos.wiki/wiki/FAQ/Pinning_Nixpkgs.
#
import (
  builtins.fetchGit {
    # Descriptive name to make the store path easier to identify.
    name = "nixpkgs-20191207-121557";
    url = https://github.com/nixos/nixpkgs/;
    # `git ls-remote https://github.com/NixOS/nixpkgs-channels nixpkgs-unstable`
    rev = "8da81465c19fca393a3b17004c743e4d82a98e4f";
  }
)
