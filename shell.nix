{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    rustup
    cargo-tarpaulin
  ];
  buildInputs = [
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
    lzma # for servicepoint2 compression
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
