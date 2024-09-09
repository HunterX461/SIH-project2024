import pymysql
import cv2

# Database connection
connection = pymysql.connect(
    host='localhost',  # Your host
    user='your_username',  # Your MySQL username
    password='your_password',  # Your MySQL password
    db='your_database_name'  # Your database name
)
user_latitude = 37.7749  #  latitude
user_longitude = -122.4194  #  longitude

try:
    cursor = connection.cursor()

    face_cascade = cv2.CascadeClassifier('haarcascade_frontalface_default.xml')

    image = cv2.imread('bilal2.jpg')

    gray_image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

    faces = face_cascade.detectMultiScale(gray_image, scaleFactor=1.1, minNeighbors=5, minSize=(10, 10))

    for (x, y, w, h) in faces:
        cv2.rectangle(image, (x, y), (x + w, y + h), (255, 0, 0), 2)
        
        # Inserting face data into the database
        cursor.execute("INSERT INTO face_data (x, y, width, height) VALUES (%s, %s, %s, %s)", (x, y, w, h))

    connection.commit()

    cv2.imshow('Detected Faces', image)

    cv2.waitKey(0)
    cv2.destroyAllWindows()

finally:
    connection.close()
