import cv2
import time
import requests
import os

CHANNEL_ID = "1224514100210569327"

def find_cam_index() -> int:
    for i in range(6):
        test_cap = cv2.VideoCapture(i)
        if test_cap.isOpened():
            test_cap.release()
            return i
    raise Exception("Could not find camera index")


def load_token() -> str:
    
    with open(".env", "r") as f:
        line = f.read().strip()
        
        return line.split("TOKEN=")[-1]

    
def main() -> None:
    
    BOT_TOKEN = load_token()
    CAM_INDEX = find_cam_index()
    cap = cv2.VideoCapture(CAM_INDEX)
    
if __name__ == "__main__":
    main()