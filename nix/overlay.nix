{ project_dir, gitignore, github_client_secret, github_client_id }:
self: super: rec {

  # oikos

  oikos_server = self.callPackage ./oikos/oikos_server.nix { };
  oikos_web = self.callPackage ./oikos/oikos_web.nix { };
  oikos_log = self.callPackage ./oikos/oikos_log.nix { };
  oikos_service = self.callPackage ./oikos/oikos_service.nix { };

  # rust

  rustChannel = self.rustChannelOf {
    channel = "1.48.0";
    sha256 = "sha256:0b56h3gh577wv143ayp46fv832rlk8yrvm7zw1dfiivifsn7wfzg";
  };
  rustStable = rustChannel.rust.override { extensions = [ "rust-src" ]; };
  rustPlatform = self.makeRustPlatform {
    rustc = rustStable;
    cargo = rustStable;
  };

  # cargo

  project_dir_gitignored = gitignoreSource project_dir;
  cargoLock = self.lib.sourceByRegex project_dir_gitignored [ "Cargo.lock" ];
  cargoToml =
    self.lib.sourceFilesBySuffices project_dir_gitignored [ "Cargo.toml" ];
  cargoVendorTarball = rerunFixed.rerunOnChange {
    preOverrideAttrs = attrs: { src = [ cargoLock ]; };
  } (rustPlatform.fetchCargoTarball {
    name = "cargo-tarball";
    src = [ cargoLock cargoToml ];
    sourceRoot = null;
    unpackPhase = ''
      for srcFile in $src; do
        cp -r $srcFile/* .
      done
    '';
    sha256 = "sha256:0a3mw4ch4rqa02fc7j7m4bf7daq3cwqcprfwxj6ccr5h85nxj280";
  });

  # tools

  rerunFixed = self.callPackage ./rerun-fixed.nix { };
  openapi-generator = self.callPackage ./openapi_generator.nix { };
  inherit (import (gitignore) { lib = self.lib; }) gitignoreSource;
  inherit project_dir;
  inherit github_client_id;
  inherit github_client_secret;
}
