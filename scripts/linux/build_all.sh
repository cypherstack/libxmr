#!/bin/bash

mkdir -p build

# echo ''$(git log -1 --pretty=format:"%H")' '$(date) >> build/git_commit_version.txt
# VERSIONS_FILE=../../lib/git_versions.dart
# EXAMPLE_VERSIONS_FILE=../../lib/git_versions_example.dart
# if [ ! -f "$VERSIONS_FILE" ]; then
#     cp $EXAMPLE_VERSIONS_FILE $VERSIONS_FILE
# fi
# COMMIT=$(git log -1 --pretty=format:"%H")
# OS="LINUX"
# sed -i "/\/\*${OS}_VERSION/c\\/\*${OS}_VERSION\*\/ const ${OS}_VERSION = \"$COMMIT\";" $VERSIONS_FILE

rm -rf build/serai
cp -rf ../../src/serai build/serai

cd build
cd serai/coins/monero
if [ "$IS_ARM" = true ]  ; then
    echo "Building arm version"
    cargo build --target aarch64-unknown-linux-gnu --release --lib
    # mkdir -p ../../../../linux/bin/aarch64-unknown-linux-gnu/release
    cp ../../target/aarch64-unknown-linux-gnu/release/libmonero_serai.so ../../../
else
    echo "Building x86_64 version"
    cargo build --target x86_64-unknown-linux-gnu --release --lib
    # mkdir -p ../../../../linux/bin/x86_64-unknown-linux-gnu/release
    cp ../../target/x86_64-unknown-linux-gnu/release/libmonero_serai.so ../../../
fi
