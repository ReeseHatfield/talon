import cv2
from datetime import datetime

cap = None


# test like 6 indexes to find the camera. I dont think there is a better way to handle this sadly
def find_cam_index() -> int:
    
    for i in range(6):
        test_cap = cv2.VideoCapture(i)
        if test_cap.isOpened():
            print(f"Camera found at index {i}")
            return i
            # cap = test_cap  
            # break
        else:
            # print(f"No camera at index {i}")
            test_cap.release()
    
    raise Exception("could not find cam index")


cam_index: int = find_cam_index() 

capture = cv2.VideoCapture(cam_index)


if capture is None or not capture.isOpened():
    print("Cannot open camera")
    exit()

ret, frame = capture.read()

if not ret:
    print("Failed to grab frame")
else:
    filename = datetime.now().strftime("%Y%m%d_%H%M%S")
    cv2.imwrite(f"img/{filename}.jpg", frame)
    cv2.imwrite("photo.jpg", frame)
    print("Saved photo.jpg")

capture.release()
cv2.destroyAllWindows()