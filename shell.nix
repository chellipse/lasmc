let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");

  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };

  # rust = pkgs.rust-bin.stable.latest.default.override {
    # extensions = [ "rust-src" "rustc-codegen-cranelift-preview" ];
  # };

  rust = pkgs.rust-bin.nightly."2024-04-02".default.override {
    extensions = [ "rust-src" "rustc-codegen-cranelift-preview" ];
  };

  # rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
    # extensions = [ "rust-src" "rustc-codegen-cranelift-preview" ];
  # });

in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
    gcc

    rust
    rust-analyzer

    ### dep ###
    # openssl
    # pkg-config
  ];

  shellHook = ''
    export NORMAL="\x1b[0m"
    export STATUS="\x1b[38;5;32m"
    export ERROR="\x1b[31m"
    export EOL="$NORMAL\n"

    export DEBUG_DIR="ignore/debug"
    export RELEASE_DIR="ignore/release"

    run() {
        gcc $1 -c -g -o $DEBUG_DIR/main.o \
        && ld $DEBUG_DIR/main.o -o $DEBUG_DIR/main \
        && ./$DEBUG_DIR/main
    }

    run_release() {
        gcc $1 -c -o $RELEASE_DIR/main.o \
        && ld $RELEASE_DIR/main.o -o $RELEASE_DIR/main \
        && ./$RELEASE_DIR/main
    }

    debug() {
        gcc $1 -c -g -o $DEBUG_DIR/main.o \
        && ld $DEBUG_DIR/main.o -o $DEBUG_DIR/main \
        && ./$DEBUG_DIR/main
        gdb ./$DEBUG_DIR/main
    }

    clean() {
        rm $DEBUG_DIR/*
        rm $RELEASE_DIR/*
    }

    dev() {
        while true; do
            inotifywait -qe modify "$1"
            printf "$STATUS$1 modified. Running...$EOL"
            run $1
            printf "\n$STATUS[End]$EOL\n"
            # printf "\n[End]\n"
        done
    }
  '';

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
