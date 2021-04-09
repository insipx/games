let
  moz_overlay =
    (import "/home/insipx/.config/nixpkgs/overlays/rust-overlay.nix");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  unstable = import (fetchTarball "channel:nixos-unstable") { };
  # rust = ((nixpkgs.rustChannelOf {
  # date = "2021-04-03";
  # channel = "nightly";
  # }).rust.override {
  # extensions = [
  #   "rust-src"
  #   # "rust-analysis"
  #   # "rustfmt-preview"
  #   "clippy-preview"
  #   "rust-std"
  # ];
  # targets = [ "wasm32-unknown-unknown" ];
  #});
  rust = (nixpkgs.latest.rustChannels.stable.rust.override { extensions = [ "rust-src" "rust-analysis" "rustfmt-preview" ]; });
in with nixpkgs;
pkgs.mkShell {
  buildInputs = [
    openssl
    pkgconfig
    nasm
    rust
    unstable.cargo-expand
    # unstable.rustracer
    unstable.cargo-edit
    unstable.cargo-udeps
    cmake
    zlib
    python3
    # llvmPackages.clang-unwrapped
    clang
    llvmPackages.libclang
    libbfd
    libopcodes
    libunwind
    autoconf
    automake
    libtool
    postgresql
    wasm-pack
    alsaLib
    xlibs.libX11
    xorg.xorgserver
    libxkbcommon
    vulkan-validation-layers
    expat
    libGL
    cargo-watch
  ];
  APPEND_LIBRARY_PATH = stdenv.lib.makeLibraryPath [
    vulkan-loader
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
    libGL
  ];

  # shellHook = ''
  # export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  # export RUSTFLAGS="-C target-cpu=native"
  # '';
  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  # RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library/";
  # PROTOC = "${protobuf}/bin/protoc";
}

