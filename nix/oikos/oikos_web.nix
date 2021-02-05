{ project_dir, gitignoreSource, stdenv }:
let src = (project_dir + "/src/rust/oikos_server");
in stdenv.mkDerivation {
  name = "oikos_web";
  inherit src;
  buildPhase = "";
  installPhase = ''
    mkdir -p $out
    ls -al
    cp -r ./static $out
  '';
}
