# Openapi

generator +OPTIONS="":
  docker run --rm -u $(id -u ${USER}):$(id -g ${USER}) -v ${PWD}:/local -w /local openapitools/openapi-generator-cli:v4.3.0 generate \
    --enable-post-process-file \
    --generate-alias-as-model \
    {{OPTIONS}}

openapi:
  #!/usr/bin/env bash
  set -e
  if [ -f .env ]; then source .env; fi
  # api
  just generator -g openapi --package-name oikos_api -i /local/src/rust/oikos_server/reference/openapi.yaml -o ./src/web/public
  openapi_generator src/openapi_templates/rust ./src/web/public/openapi.json -d ./src/rust/oikos_api
  cargo fmt

build:
  #!/usr/bin/env bash
  cd src/rust/oikos_web
  wasm-pack build --target web --out-name wasm --out-dir ./static