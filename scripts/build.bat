:: Run from the project root.
@echo off
cargo build --release ^
    && gzip --stdout target/release/ned.exe > artifacts/ned.windows.gz ^
    || exit /b 1
echo Artifacts written to ./artifacts.
