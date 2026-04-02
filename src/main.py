import cv2
import time
import requests
import os


# I hate this, but i feel like it might be the best way sadly
# cam index on my laptop is like 3
# cam index on my laptop islike 20 something
def find_cam_index() -> int:
    for i in range(25):
        test_cap = cv2.VideoCapture(i)
        if test_cap.isOpened():
            test_cap.release()
            return i
    raise Exception("Could not find camera index")


def load_token() -> str:
    
    with open(".env", "r") as f:
        line = f.read().strip()
        
        return line.split("TOKEN=")[-1]


def check_time(last_capture) -> bool:
    now = time.time()
    return now - last_capture > 30


def clear_cap_buffer(cap) -> None:
    for _ in range(5): cap.grab()
    

def take_pic(cap) -> str:
    clear_cap_buffer(cap);
    
    _, img = cap.retrieve()
    
    path = "snap.jpg"
    cv2.imwrite(path, img)
    
    return path


def build_header():
    return {
        "Authorization": f"Bot {BOT_TOKEN}"
    }


def send_image_to_discord(img_path: str, headers): 
    
    with open(img_path, "rb") as f:
        url = f"https://discord.com/api/v10/channels/{CHANNEL_ID}/messages"
        r = requests.post(url, headers=headers, files={"file": f})
        if r.status_code == 200:
            print("Sent to Discord.")
        else:
            print(f"Error: {r.status_code} - {r.text}")

import datetime
import time

def is_dark_in_dayton():
    now = datetime.datetime.now()
    
    decimal_minute = now.minute / 60
    
    #  hour as a decimal + minute value (as int) to decimal
    decimal_hour = now.hour + decimal_minute
    
    
    is_in_light_window = 7.0 <= decimal_hour and decimal_hour <= 19.5 
    # 7 am to 730 pm
    return not is_in_light_window




# todo read me in
CHANNEL_ID = "1224514100210569327"
BOT_TOKEN = load_token()
CAM_INDEX = find_cam_index()
MOTION_THRESHOLD_PX = 10000

def main() -> None:
    cap = cv2.VideoCapture(CAM_INDEX)
    last_capture = 0
    headers = build_header()

    print("monitoring for motion...")

    while True:
        if is_dark_in_dayton():
            # saves on my power bill
            time.sleep(200)
            continue
        
        ret, frame1 = cap.read()
        time.sleep(0.2)
        ret, frame2 = cap.read()

        if not ret:
            break

        gray1 = cv2.cvtColor(frame1, cv2.COLOR_BGR2GRAY)
        gray2 = cv2.cvtColor(frame2, cv2.COLOR_BGR2GRAY)
        gray1 = cv2.GaussianBlur(gray1, (21, 21), 0)
        gray2 = cv2.GaussianBlur(gray2, (21, 21), 0)

        delta = cv2.absdiff(gray1, gray2)
        thresh = cv2.threshold(delta, 25, 255, cv2.THRESH_BINARY)[1]
        thresh = cv2.dilate(thresh, None, iterations=2)
        
        motion_score = cv2.countNonZero(thresh)

        if motion_score > MOTION_THRESHOLD_PX:
            if check_time(last_capture):
                print(f"Motion Detected ({motion_score}). Taking pic...")
                time.sleep(1)
                img_path = take_pic(cap)
                send_image_to_discord(img_path, headers)
                last_capture = time.time()

    cap.release()
if __name__ == "__main__":
    main()