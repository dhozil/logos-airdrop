@echo off
REM End-to-end demo script for LP-0003 on Windows
REM Requires: LEZ wallet, airdrop program deployed

set PROGRAM_ID=%1
if "%PROGRAM_ID%"=="" set PROGRAM_ID=<DEPLOYED_PROGRAM_ID>

echo === LP-0003 Demo: Private Airdrop Distributor ===
echo Program ID: %PROGRAM_ID%
echo.

REM Step 1: Check wallet health
echo 1. Checking wallet health...
wallet check-health
if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%

REM Step 2: Create sample recipients CSV
echo 2. Creating sample recipients...
echo address,amount> %TEMP%\recipients.csv
echo 0x6a756c69616e0000000000000000000000000000000000000000000000000001,1000>> %TEMP%\recipients.csv
echo 0x6a756c69616e0000000000000000000000000000000000000000000000000002,2000>> %TEMP%\recipients.csv
echo 0x6a756c69616e0000000000000000000000000000000000000000000000000003,3000>> %TEMP%\recipients.csv
echo 0x6a756c69616e0000000000000000000000000000000000000000000000000004,4000>> %TEMP%\recipients.csv
echo 0x6a756c69616e0000000000000000000000000000000000000000000000000005,5000>> %TEMP%\recipients.csv

REM Step 3: Generate distribution manifest
echo 3. Generating distribution manifest...
airdrop-cli generate --csv %TEMP%\recipients.csv --token <TOKEN_PROGRAM_ID> --distributor <DISTRIBUTOR_ADDRESS> --allocation 15000 --output %TEMP%\distribution.json

echo.
echo === Demo setup complete. Ready for on-chain operations. ===
