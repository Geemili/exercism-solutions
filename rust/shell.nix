let
  moz_overlay = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  );
in

{ pkgs ? import <nixpkgs> { overlays = [ moz_overlay ]; } }:
with pkgs;

stdenv.mkDerivation {
  name = "augr-env";
  buildInputs = [
    git
    rustChannels.stable.rust
  ];
}
