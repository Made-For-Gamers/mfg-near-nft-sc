@echo off

cd ..
title MFG NFT build
cargo build --all --target wasm32-unknown-unknown --release
xcopy %CD%\target\wasm32-unknown-unknown\release\*.wasm %CD%\res /Y
pause