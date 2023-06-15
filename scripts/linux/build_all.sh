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

cd ../../
if [ "$IS_ARM" = true ]  ; then
    echo "Building arm libxmr"
    cargo build --target aarch64-unknown-linux-gnu --release --lib
    cp target/aarch64-unknown-linux-gnu/release/liblibxmr.so scripts/linux/build/libxmr.so
else
    echo "Building x86_64 libxmr"
    cargo build --target x86_64-unknown-linux-gnu --release --lib
    cp target/x86_64-unknown-linux-gnu/release/liblibxmr.so scripts/linux/build/libxmr.so
fi