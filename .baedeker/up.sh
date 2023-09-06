#!/bin/sh
set -e
BDK_DIR=$(dirname $(readlink -f "$0"))
RUST_LOG=info baedeker --spec=docker -J$BDK_DIR --generator=docker_compose=$BDK_DIR/.bdk-env --generator=docker_compose_discover=$BDK_DIR/.bdk-env/discover.env --secret=file=$BDK_DIR/.bdk-env/secret  --tla-str=relay_spec=rococo-local --input-modules='lib:baedeker-library/ops/nginx.libsonnet' --input-modules='lib:baedeker-library/ops/devtools.libsonnet' $@ $BDK_DIR/rewrites.jsonnet
cd $BDK_DIR/.bdk-env
docker compose up -d --wait --remove-orphans
