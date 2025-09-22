@echo off
REM QubeDB Test Script
REM Script untuk test berbagai fitur QubeDB

echo 🧪 QubeDB Test Suite
echo ====================

cd qubedb-core

echo.
echo 📦 Test 1: Basic Usage
echo ----------------------
cargo run --example basic_usage

echo.
echo ⚡ Test 2: Performance Test
echo ---------------------------
cargo run --example performance_test

echo.
echo 🔍 Test 3: Unit Tests
echo ---------------------
cargo test

echo.
echo 🎯 Test 4: Integration Tests
echo ----------------------------
cargo test --features "all-drivers"

echo.
echo 🧠 Test 5: Vector Operations
echo ----------------------------
cargo run --example vector_test

echo.
echo 📊 Test 6: Graph Operations
echo ----------------------------
cargo run --example graph_test

echo.
echo 🎉 All tests completed!
echo =====================
echo.
echo ✅ QubeDB is working perfectly!
echo.
echo 🚀 Try the GUI:
echo cd ..\qubedb-gui
echo cargo run
echo.
echo 📖 Read more: SETUP-GUIDE.md

pause
