#!/usr/bin/env bash

# This script builds the NSS3 library (with NSPR) for Desktop.

set -euvx

if [ "$#" -lt 1 -o "$#" -gt 2 ]
then
  echo "Usage:"
  echo "./build-nss-desktop.sh <NSS_SRC_PATH> [CROSS_COMPILE_TARGET]"
  exit 1
fi

NSS_SRC_PATH=$1
# Whether to cross compile from Linux to a different target.  Really
# only intended for automation.
CROSS_COMPILE_TARGET=${2-}

if [ -n "$CROSS_COMPILE_TARGET" -a $(uname -s) != "Linux" ]; then
  echo "Can only cross compile from 'Linux'; 'uname -s' is $(uname -s)"
  exit 1
fi

if [[ "$CROSS_COMPILE_TARGET" =~ "win32-x86-64" ]]; then
  NSS_DIR=$(abspath "desktop/win32-x86-64/nss")
elif [[ "$CROSS_COMPILE_TARGET" =~ "darwin" ]]; then
  NSS_DIR=$(abspath "desktop/darwin/nss")
elif [ -n "$CROSS_COMPILE_TARGET" ]; then
  echo "Cannot build NSS for unrecognized target OS $CROSS_COMPILE_TARGET"
  exit 1
elif [ $(uname -s) == "Darwin" ]; then
  NSS_DIR=$(abspath "desktop/darwin/nss")
elif [ $(uname -s) == "Linux" ]; then
  # This is a JNA weirdness: "x86-64" rather than "x86_64".
  NSS_DIR=$(abspath "desktop/linux-x86-64/nss")
else
   echo "Cannot build NSS on unrecognized host OS $(uname -s)"
   exit 1
fi

if [ -d "$NSS_DIR" ]; then
  echo "$NSS_DIR folder already exists. Skipping build."
  exit 0
fi

# NSPR_DIST=$(mktemp -d)
# pushd $NSPR_DIST
# $NSS_SRC_PATH/nspr/configure --enable-64bit --disable-debug --enable-optimize
# make
# popd

# TODO: other platforms/cross compilation, this is only macOS
make \
  USE_64=1 \
  BUILD_OPT=1 \
  NSS_DISABLE_CHACHAPOLY=1 \
  NSS_DISABLE_DBM=1 \
  nss_build_all \
  -C $NSS_SRC_PATH/nss

mkdir -p "$NSS_DIR/include/nss"
mkdir -p "$NSS_DIR/lib"
cp -p -L "$NSS_SRC_PATH/dist/Darwin18.2.0_cc_64_OPT.OBJ/lib/libnss3.dylib" "$NSS_DIR/lib"
cp -p -L "$NSS_SRC_PATH/dist/Darwin18.2.0_cc_64_OPT.OBJ/lib/libnssutil3.dylib" "$NSS_DIR/lib"
cp -p -L "$NSS_SRC_PATH/dist/Darwin18.2.0_cc_64_OPT.OBJ/lib/libplc4.dylib" "$NSS_DIR/lib"
cp -p -L "$NSS_SRC_PATH/dist/Darwin18.2.0_cc_64_OPT.OBJ/lib/libplds4.dylib" "$NSS_DIR/lib"
cp -p -L "$NSS_SRC_PATH/dist/Darwin18.2.0_cc_64_OPT.OBJ/lib/libnspr4.dylib" "$NSS_DIR/lib"
cp -p -L "$NSS_SRC_PATH/dist"/public/nss/* "$NSS_DIR/include/nss"
