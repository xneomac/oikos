{ project_dir, lib, gitignoreSource, stdenv }:
let src = lib.sourceByRegex project_dir [ "log4rs.yml" ];
in stdenv.mkDerivation {
  name = "oikos_log";
  inherit src;
  buildPhase = "";
  installPhase = ''
    mkdir -p $out
    cp log4rs.yml $out
  '';
}
