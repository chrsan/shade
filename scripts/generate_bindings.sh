#!/bin/bash

BASE_DIR=`cd $(dirname ${BASH_SOURCE[0]}) && pwd`

pushd $BASE_DIR/.. > /dev/null

bindgen "./cc/core.h" \
    --no-layout-tests \
    --no-prepend-enum-name \
    --size_t-is-usize \
    --with-derive-default \
    --whitelist-function shade.\* \
    -o "./src/core_bindings.rs"

bindgen "./cc/skottie.h" \
    --no-layout-tests \
    --no-prepend-enum-name \
    --size_t-is-usize \
    --with-derive-default \
    --opaque-type ShadeCanvas \
    --raw-line 'use crate::core_bindings::ShadeCanvas;' \
    --whitelist-function shade_skottie.\* \
    -o "./src/skottie_bindings.rs"

bindgen "./cc/svg.h" \
    --no-layout-tests \
    --no-prepend-enum-name \
    --size_t-is-usize \
    --with-derive-default \
    --opaque-type ShadeCanvas \
    --raw-line 'use crate::core_bindings::ShadeCanvas;' \
    --whitelist-function shade_svg.\* \
    -o "./src/svg_bindings.rs"
