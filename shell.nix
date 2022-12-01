with import <nixpkgs> {};
let
  inputs = [
    libtool
    rustc
    cargo
    gcc
  ];

in mkShell {
  buildInputs = inputs;
  nativeBuildInputs = with pkgs; [ rustc cargo gcc libpcap];
  shellHook = ''
  '';
  RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}