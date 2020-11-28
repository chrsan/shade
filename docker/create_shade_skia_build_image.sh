#!/bin/bash

set -e

BASE_DIR=`cd $(dirname ${BASH_SOURCE[0]}) && pwd`

pushd $BASE_DIR > /dev/null

docker build -t shade_skia_build .
