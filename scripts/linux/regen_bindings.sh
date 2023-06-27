#!/bin/bash

cd ../..

cbindgen --config cbindgen.toml --crate libxmr --output libxmr_bindings.h
dart run ffigen --config ffigen.yaml
