#include "SoftwareSerial.h"
#include "TinyGPS++.h"
#include "log/Logger.h"
#include "radio/Connection.h"
#include <Arduino.h>

static radio::Connection connection;
static Logger logger("main");
static SoftwareSerial gpsSerial(9, 9);
static TinyGPSPlus gps;

void setup() {
  Serial.begin(9600);  // Serial port to computer
  Serial1.begin(9600); // Serial port to HC1
  gpsSerial.begin(9600);
  delay(1000);
  logger.info("Ready");
}

void loop() {
  connection.recv();
  while (gpsSerial.available()) {
    gps.encode(gpsSerial.read());
  }

  if (gps.satellites.isUpdated()) {
    logger.info("Satellite list updated");
    Serial.print("Connected to ");
    Serial.print(gps.satellites.value());
    Serial.println(" satellites");
  }

  if (gps.speed.isUpdated()) {
    logger.info("Speed updated");
    Serial.print(gps.speed.mps());
    Serial.println(" m/s");
  }

  if (gps.location.isUpdated()) {
    logger.info("Location updated");
    Serial.print("LAT=");
    Serial.print(gps.location.lat());
    Serial.print(" LON=");
    Serial.println(gps.location.lng());
  }
}
