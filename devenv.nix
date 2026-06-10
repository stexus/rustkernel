{ pkgs, ... }: {

  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
      "llvm-tools"
      "rust-analyzer"
    ];
  };

  packages = [ pkgs.qemu ];

}
