cd img
source venv/bin/activate

pip install -r requirements.txt

python3 test.py $@

deactivate
