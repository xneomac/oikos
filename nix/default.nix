{ nixpkgs ? import ./nixpkgs.nix, project_dir ? <project_dir>, github_client_id
, github_client_secret }:
let pkgs = import nixpkgs { };
in let
  moz_overlay = pkgs.fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    rev = "efda5b357451dbb0431f983cca679ae3cd9b9829";
    sha256 = "sha256:11wqrg86g3qva67vnk81ynvqyfj0zxk83cbrf0p9hsvxiwxs8469";
  };
  nix-ros-overlay = pkgs.fetchFromGitHub {
    owner = "lopsided98";
    repo = "nix-ros-overlay";
    rev = "25d7c3029420974d8f2ad0411e615a69c57ecc39";
    sha256 = "sha256:0d9zxgfrzfv3rjnyy9m5hqlway5pn1xplvfscc67ggi9w13m8zwd";
  };
  gitignore = pkgs.fetchFromGitHub {
    owner = "hercules-ci";
    repo = "gitignore";
    rev = "f9e996052b5af4032fe6150bba4a6fe4f7b9d698";
    sha256 = "sha256:0jrh5ghisaqdd0vldbywags20m2cxpkbbk5jjjmwaw0gr8nhsafv";
  };
in import (nix-ros-overlay) {
  inherit nixpkgs;
  inherit moz_overlay;
  inherit nix-ros-overlay;
  overlays = [
    (import (moz_overlay))
    (import ./overlay.nix {
      inherit github_client_id;
      inherit github_client_secret;
      inherit project_dir;
      inherit gitignore;
    })
  ];
}
