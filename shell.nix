{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
  ];

  buildInputs = with pkgs; [
    alsa-lib
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [ alsa-lib ]);
}
