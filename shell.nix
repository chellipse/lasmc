let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");

  pkgs = import <nixos-unstable> { overlays = [ rust_overlay ]; };

  rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
    extensions = [ "rust-src" "rustc-codegen-cranelift-preview" ];
  });

in
  pkgs.mkShell {
    nativeBuildInputs = [

    rust
    pkgs.rust-analyzer

    ### dep ###
    # openssl
    # pkg-config
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
