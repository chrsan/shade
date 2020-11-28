#!/bin/bash

set -e

BASE_DIR=`cd $(dirname ${BASH_SOURCE[0]}) && pwd`
OS=`uname -s | tr '[:upper:]' '[:lower:]'`
ARCH=`uname -m`
SKIA_COMMIT="b05f80697afd6e8cb2389b77c4e20d88caa30393"

pushd $BASE_DIR/.. > /dev/null

SKIA_CLONED=0
if [[ ! -d skia ]]; then
    SKIA_CLONED=1
    echo 'cloning skia'
    git clone --quiet --no-checkout https://github.com/google/skia.git
    pushd skia > /dev/null
    git checkout --quiet "$SKIA_COMMIT"
    popd > / dev/null
fi

pushd skia > /dev/null

if [[ "$1" == "sync" || "$SKIA_CLONED" -eq 1 ]]; then
    echo 'creating a custom DEPS file'

    set +e

    echo 'deps = {' > DEPS.shade
    while IFS= read -r LINE; do
        echo "$LINE" | grep -e 'freetype' \
            -e 'harfbuzz' \
            -e 'icu' \
            -e 'libjpeg-turbo' \
            -e 'libpng' \
            -e 'zlib' \
            -e 'expat' > /dev/null
        if [[ "$?" -eq 0 ]]; then
            echo "  $LINE" >> DEPS.shade
        fi
    done < DEPS
    echo '}' >> DEPS.shade

    set -e

    echo 'syncing deps'
    GIT_SYNC_DEPS_PATH="DEPS.shade" tools/git-sync-deps
fi

# Let's piggy back on the Skia build for our C++ sources.
git reset --hard > /dev/null

pushd modules > /dev/null

rm shade_capi 2>&1 > /dev/null || true
ln -s "${BASE_DIR}/../cc" shade_capi

echo 'creating gn shade_capi group'
echo 'group("shade_capi") {' >> ../BUILD.gn
echo '  deps = [' >> ../BUILD.gn
echo '    "modules/shade_capi",' >> ../BUILD.gn
echo '  ]' >> ../BUILD.gn
echo '}' >> ../BUILD.gn
popd >> /dev/null

BUILD_DIR="${BASE_DIR}/../out/${OS}/${ARCH}"
if [[ ! -d "$BUILD_DIR" ]]; then
    ./bin/gn gen "$BUILD_DIR" --args="cc=\"clang\" cxx=\"clang++\" \
      extra_cflags=[\"-DSK_RELEASE\"]
      is_debug=false \
      is_official_build=true \
      is_component_build=false \
      werror=true \
      skia_compile_processors = false \
      skia_enable_api_available_macro = false \
      skia_enable_android_utils = false \
      skia_enable_ccpr = false \
      skia_enable_discrete_gpu = false \
      skia_enable_flutter_defines = false \
      skia_enable_fontmgr_empty = false
      skia_enable_fontmgr_fuchsia = false \
      skia_enable_fontmgr_win = false \
      skia_enable_fontmgr_win_gdi = false \
      skia_enable_gpu = false \
      skia_enable_pdf = false \
      skia_enable_skottie = true \
      skia_enable_skrive = false \
      skia_enable_sksl_interpreter = false \
      skia_enable_skvm_jit_when_possible = false \
      skia_enable_tools = false \
      skia_enable_gpu_debug_layers = false \
      skia_generate_workarounds = false \
      skia_include_multiframe_procs = false \
      skia_lex = false \
      skia_pdf_subset_harfbuzz = false \
      skia_use_angle = false \
      skia_use_dawn = false \
      skia_use_direct3d = false \
      skia_use_egl = false \
      skia_use_gl = false \
      skia_use_expat = true \
      skia_use_system_expat = false \
      skia_use_experimental_xform = false \
      skia_use_ffmpeg = false \
      skia_use_fixed_gamma_text = false \
      skia_use_fontconfig = false \
      skia_use_fonthost_mac = false \
      skia_use_freetype = true \
      skia_use_system_freetype2=false \
      skia_use_harfbuzz = true \
      skia_use_system_harfbuzz=false \
      skia_use_icu = true \
      skia_use_system_icu=false \
      skia_use_libheif = false \
      skia_use_libjpeg_turbo_decode = true \
      skia_use_libjpeg_turbo_encode = false \
      skia_use_system_libjpeg_turbo=false \
      skia_use_libpng_decode = true \
      skia_use_libpng_encode = false \
      skia_use_system_libpng=false \
      skia_use_libwebp_decode = false \
      skia_use_libwebp_encode = false \
      skia_use_lua = false \
      skia_use_metal = false \
      skia_use_ndk_images = false \
      skia_use_opencl = false \
      skia_use_piex = false \
      skia_use_sfml = false \
      skia_use_webgl = false \
      skia_use_wuffs = false \
      skia_use_x11 = false \
      skia_use_xps = false \
      skia_use_zlib = true \
      skia_use_system_zlib=false \
      skia_use_vulkan = false \
      skia_build_fuzzers = false \
      skia_compile_sksl_tests = false \
      skia_enable_fontmgr_android = false \
      skia_enable_fontmgr_custom_directory = false \
      skia_enable_fontmgr_custom_embedded = false \
      skia_enable_fontmgr_custom_empty = skia_use_freetype \
      skia_enable_fontmgr_fontconfig = false \
      skia_enable_fontmgr_FontConfigInterface = false \
      skia_enable_nvpr = false \
      skia_enable_spirv_validation = false \
      skia_use_dng_sdk = false \
      skia_use_libgifcodec = false \
      skia_use_sfntly = false \
      skia_enable_vulkan_debug_layers = false \
      skia_enable_direct3d_debug_layer = false \
      skia_use_vma = false \
      skia_enable_svg = true"
fi

ninja -C "$BUILD_DIR" libskia.a libskshaper.a libskottie.a libsksg.a libsvg.a shade_capi

LIB_DIR="${BASE_DIR}/../lib/${OS}/${ARCH}"
mkdir -p "$LIB_DIR" > /dev/null
pushd "$BUILD_DIR" > /dev/null
cp libskia.a libskshaper.a libskottie.a libsksg.a libsvg.a libshade_capi.a "$LIB_DIR"
