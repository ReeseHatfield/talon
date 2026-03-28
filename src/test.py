import cv2
import sys
from datetime import datetime


CAM_INDEX = int(sys.argv[1])
cap = cv2.VideoCapture(2)

if not cap.isOpened():
    print("Cannot open camera")
    exit()

# read frame
ret, frame = cap.read()

if not ret:
    print("Failed to grab frame")
else:
    # backup dir
    cv2.imwrite(f"img/{datetime.now()}.jpg", frame)
    # easy most recent photo
    cv2.imwrite("photo.jpg", frame)
    print("Saved photo.jpg")

cap.release()
cv2.destroyAllWindows()
