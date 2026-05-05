#!/bin/bash

git pull --rebase

pip install --upgrade pip


if command -v python3.12 >/dev/null 2>&1; then
    PYTHON=python3.12
else
    PYTHON=python3
fi

if [ ! -d "venv" ]; then
    $PYTHON -m venv venv
fi

source venv/bin/activate

pip install -r requirements.txt

nohup $PYTHON -u src/main.py > talon.out &