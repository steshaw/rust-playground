#!/usr/bin/env bash

set -euo pipefail

for i in {1..100}; do
  if [[ $((i % 3)) = 0 ]]; then
    echo -n "fizz"
  fi
  if [[ $((i % 5)) = 0 ]]; then
    echo -n "buzz"
  fi
  if [[ $((i % 3)) != 0 ]] && [[ $((i % 5)) != 0 ]]; then
    echo -n "$i"
  fi
  echo
done
