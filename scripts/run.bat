@echo off

set currentdir=%cd%

docker build -t devraymondsh/rust_web_frameworks_benchmark .

docker run  --rm -it ^
    -e CARGO_HOME="/.cargo" ^
    -v="%USERPROFILE%:/.cargo" ^
    -v="%currentdir%/scripts:/rust_web_frameworks_benchmark/scripts" ^
    -v="%currentdir%/frameworks:/rust_web_frameworks_benchmark/frameworks" ^
    -v="%currentdir%/benchmarking_log:/rust_web_frameworks_benchmark/benchmarking_log" ^
    devraymondsh/rust_web_frameworks_benchmark