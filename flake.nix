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
    system = "x86_64-linux";

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
  in {
    devShells.${system}.default = pkgs.mkShell {
      name = "rustkernel";

      packages = [
        rust-toolchain
        pkgs.qemu
        pkgs.gdb
      ];

      # Teach rust-analyzer where the standard library source lives.
      RUST_SRC_PATH = "${pkgs.fenix.complete.rust-src}/lib/rustlib/src/rust/library";
    };
  };
}
