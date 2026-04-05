#!/bin/bash



python3 -m venv venv

source venv/bin/activate

pip install -r requirements.txt

echo "Running TALON as a long running server process"

nohup python3 src/main.py > talon.out &
