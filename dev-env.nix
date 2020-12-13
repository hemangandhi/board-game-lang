with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "board-game-lang-env";
  buildInputs = [
    # editor
    emacs26Packages.company
    emacs26Packages.rustic
    emacs26Packages.flycheck
    emacs26Packages.eglot
    emacs26Packages.cargo
    # rust linter
    clippy
    # rust builder
    cargo
    # rust compiler
    rustc
    # rust formatter
    rustfmt
    # JS deps
    nodejs-12_x
    yarn
    nodePackages."@angular/cli"
  ];
}
