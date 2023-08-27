#!/usr/bin/env bash
set -euo pipefail

dfx canister create configuration --no-wallet
dfx canister create configuration --no-wallet
dfx canister create configuration --no-wallet

dfx build individual_user
dfx build user_index
dfx build configuration

dfx canister install configuration --mode auto