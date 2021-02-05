{ project_dir, lib, gitignoreSource, stdenv, oikos_web, oikos_log, oikos_server
, github_client_id, github_client_secret }:
let src = gitignoreSource (project_dir + "/src/systemd");
in stdenv.mkDerivation {
  name = "oikos_service";
  inherit src;
  prePatch = ''
    substituteInPlace oikos.service --replace 'APP_PATH=app_path' 'APP_PATH=${oikos_web}/static'
    substituteInPlace oikos.service --replace 'LOG_CONFIG_FILE=log4rs.yml' 'LOG_CONFIG_FILE=${oikos_log}/log4rs.yml'
    substituteInPlace oikos.service --replace 'ExecStart=oikos_server' 'ExecStart=${oikos_server}/bin/oikos_server'
    substituteInPlace oikos.service --replace 'GITHUB_CLIENT_ID=github_client_id' 'GITHUB_CLIENT_ID=${github_client_id}'
    substituteInPlace oikos.service --replace 'GITHUB_CLIENT_SECRET=github_client_secret' 'GITHUB_CLIENT_SECRET=${github_client_secret}'
  '';
  buildPhase = "";
  installPhase = ''
    mkdir -p $out/lib/systemd/system/
    cp oikos.service $out/lib/systemd/system/
  '';
}
