{ project_dir, gitignoreSource, rustPlatform, stdenv, openssl, pkgconfig, udev
, dbus, llvmPackages, clang, cmake, glibcLocales, cargoVendorTarball }:
let
  src = gitignoreSource (project_dir + "/src/rust/oikos_server");
  oikos_api_src = gitignoreSource (project_dir + "/src/rust/oikos_api");
  oikos_web_src = gitignoreSource (project_dir + "/src/rust/oikos_web");
  open_recipe_format_src =
    gitignoreSource (project_dir + "/src/rust/open_recipe_format");
  uniqdb_src = gitignoreSource (project_dir + "/src/rust/uniqdb");
  cargoPatches = ''
    [patch.crates-io]
    oikos_api = { path = \"${oikos_api_src}\" }
    oikos_web = { path = \"${oikos_web_src}\" }
    open_recipe_format = { path = \"${open_recipe_format_src}\" }
    uniqdb = { path = \"${uniqdb_src}\" }
  '';
in rustPlatform.buildRustPackage rec {
  name = "oikos_server";
  inherit src;
  cargoVendorDir = "@vendor@";
  prePatch = ''
    mkdir ${cargoVendorDir}
    unpackFile ${cargoVendorTarball}
    mv ${cargoVendorTarball.name}/* ${cargoVendorDir}
    echo "${cargoPatches}" >> Cargo.toml
    cargo generate-lockfile
  '';
  nativeBuildInputs = [ pkgconfig cmake clang ];
  buildInputs = [ openssl udev dbus cmake glibcLocales ];
  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  verifyCargoDeps = false;
  cargoBuildFlags = [ ];
  doCheck = false;
  meta = with stdenv.lib; {
    description = "oikos_server";
    license = licenses.unlicense;
    platforms = platforms.all;
  };
}
