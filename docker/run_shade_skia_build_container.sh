#!/bin/bash

set -e

BASE_DIR=`cd $(dirname ${BASH_SOURCE[0]}) && pwd`

pushd $BASE_DIR/.. > /dev/null

docker run -ti --name shade_skia_build -v "$PWD":/workspace -w /workspace shade_skia_build /bin/bash
