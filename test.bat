@echo off
REM Start Rust server
start "" cargo run --release

REM Wait for the server to start 
:CHECK_SERVER
curl -s -S http://127.0.0.1:3690/get > nul
if %errorlevel% neq 0 (
    echo Waiting for the server to start...
    timeout /t 10 > nul
    goto CHECK_SERVER
)

REM Open HTML page in default web browser
start "RUST CRUD API" "index.html"
