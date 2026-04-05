#!/bin/bash


set -e

echo "Running TALON as a long running server process"

nohup python3 src/main.py > talon.out &
