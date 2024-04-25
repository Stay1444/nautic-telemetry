#include "SoftwareSerial.h"
#include "TinyGPS++.h"
#include <Arduino.h>

static TinyGPSPlus gps;
static SoftwareSerial gpsSerial(5, 6);

void setup() {
  Serial.begin(9600); // Serial port to computer
  // Serial1.begin(9600); // Serial port to HC1
  gpsSerial.begin(9600);
  delay(1000);
}

void loop() {
  Serial.println(gpsSerial.available());

  while (gpsSerial.available()) {
    Serial.println("Got data from serial");
    gps.encode(gpsSerial.read());
    if (gps.location.isUpdated()) {
      Serial.print("Latitude= ");
      Serial.print(gps.location.lat(), 6);
      Serial.print(" Longitude= ");
      Serial.println(gps.location.lng(), 6);
      Serial.print("Speed in m/s = ");
      Serial.println(gps.speed.mps());
    }
  }
}
