let
  here = toString ./.;
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust = (pkgs.rustChannelOf {
    channel = "stable";
    date = "2020-08-27";
  }).rust.override {
    extensions = [ "rust-src" "rust-analysis" ];
    targets = [ "wasm32-unknown-unknown" ];
  };
  rustPlatform = pkgs.makeRustPlatform {
    rustc = rust;
    cargo = rust;
  };
  openapi-generator =
    pkgs.callPackage ./nix/openapi_generator.nix { inherit rustPlatform; };
in pkgs.mkShell {
  buildInputs = [
    pkgs.python37Packages.pip
    pkgs.python37Packages.setuptools
    pkgs.sccache
    openapi-generator
    pkgs.openssl
    pkgs.pkg-config
    rust
    pkgs.wasm-pack
  ];
  PIP_PREFIX = "${here}/build/pip_packages";
  PYTHONPATH =
    "${here}/build/pip_packages/lib/python3.7/site-packages:$PYTHONPATH";
  SOURCE_DATE_EPOCH = "";
  shellHook = ''
    export PATH="$PATH:$(pwd)/build/pip_packages/bin"
  '';
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
  LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
  CARGO_INCREMENTAL = 1;
  RUSTC_WRAPPER = "sccache";
}

