{
  description = "Rust kernel dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
  }: let
    systems = ["x86_64-linux" "aarch64-darwin"];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
  in {
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };

      rust-toolchain = pkgs.fenix.combine [
        (pkgs.fenix.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "llvm-tools"
          "rust-analyzer"
        ])
        pkgs.fenix.targets.aarch64-unknown-none.latest.rust-std
      ];

      # gdb is unavailable on Darwin; fall back to lldb.
      debugger =
        if pkgs.stdenv.isDarwin
        then pkgs.lldb
        else pkgs.gdb;
    in {
      default = pkgs.mkShell ({
          name = "rustkernel";

          packages =
            [
              rust-toolchain
              pkgs.qemu
              debugger
              pkgs.dtc
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              # Provides ld.lld with correct Nix-store rpaths (see CARGO_TARGET_… below).
              pkgs.llvmPackages_latest.lld
            ];

          # Teach rust-analyzer where the standard library source lives.
          RUST_SRC_PATH = "${pkgs.fenix.complete.rust-src}/lib/rustlib/src/rust/library";
        }
        // pkgs.lib.optionalAttrs pkgs.stdenv.isDarwin {
          # The rust-lld bundled with the fenix toolchain is a macOS binary built on
          # the Rust CI runner; its @rpath points at /Users/runner/… which doesn't
          # exist here.  Tell Cargo to use the Nix-packaged ld.lld instead.
          CARGO_TARGET_AARCH64_UNKNOWN_NONE_LINKER = "ld.lld";
        });
    });
  };
}
