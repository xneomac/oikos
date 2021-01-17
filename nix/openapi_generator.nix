{ rustPlatform, fetchFromGitHub }:

rustPlatform.buildRustPackage rec {
  name = "openapi-generator";
  src = builtins.fetchGit {
    url = "https://gitlab.com/easymov/openapi_generator.git";
    rev = "9b3c723cb14e7e9ff275bf031b5ca6839a42fea8";
    ref = "master";
  };
  cargoSha256 = "sha256:1gaa6wlqzm4m9j5nm08i9bm22x7z51d6r8kac6b0sq7jnw0sy80r";
  doCheck = false;
}
