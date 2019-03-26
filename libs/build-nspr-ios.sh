#!/usr/bin/env bash

# This script builds NSPR for iOS.

set -euvx

if [ "$#" -ne 4 ]
then
    echo "Usage:"
    echo "./build-nspr-ios.sh <ABSOLUTE_SRC_DIR> <DIST_DIR> <ARCH> <IOS_MIN_SDK_VERSION>"
    exit 1
fi

NSPR_SRC_DIR=$1
DIST_DIR=$2
ARCH=$3
IOS_MIN_SDK_VERSION=$4

if [[ "$ARCH" == "x86_64" ]]; then
  OS_COMPILER="iPhoneSimulator"
  TARGET="x86_64-apple-darwin"
elif [[ "$ARCH" == "arm64" ]]; then
  OS_COMPILER="iPhoneOS"
  TARGET="aarch64-apple-darwin"
else
  echo "Unsupported architecture"
  exit 1
fi

NSPR_IOS="$DIST_DIR/$ARCH"
mkdir -p "$NSPR_IOS"
pushd "$NSPR_IOS"

DEVELOPER=$(xcode-select -print-path)
CROSS_TOP="$DEVELOPER/Platforms/$OS_COMPILER.platform/Developer"
CROSS_SDK="$OS_COMPILER.sdk"
TOOLCHAIN_BIN="$DEVELOPER/Toolchains/XcodeDefault.xctoolchain/usr/bin"
export STRIP="$TOOLCHAIN_BIN/strip"
export RANLIB="$TOOLCHAIN_BIN/ranlib"
export AR="$TOOLCHAIN_BIN/ar"
export AS="$TOOLCHAIN_BIN/as"
export LD="$TOOLCHAIN_BIN/ld -arch $ARCH"
export CC="$TOOLCHAIN_BIN/clang -arch $ARCH -isysroot $CROSS_TOP/SDKs/$CROSS_SDK -mios-version-min=$IOS_MIN_SDK_VERSION"
export CCC="$CC"

$NSPR_SRC_DIR/configure --target=$TARGET --disable-debug --enable-optimize
make

popd
