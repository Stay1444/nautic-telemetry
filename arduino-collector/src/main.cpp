#include "SoftwareSerial.h"
#include "TinyGPS++.h"
#include "log/Logger.h"
#include "radio/Connection.h"
#include "radio/packets/Master.h"
#include "radio/packets/Slave.h"
#include <Arduino.h>

static radio::Connection connection;
static Logger logger("main");
static SoftwareSerial gpsSerial(9, 9);
static TinyGPSPlus gps;

using namespace radio;

void setup() {
  Serial.begin(9600);  // Serial port to computer
  Serial1.begin(9600); // Serial port to HC1
  gpsSerial.begin(9600);
  delay(1000);
  logger.info("Ready");
}

void loop() {
  radio::Packet *packet = connection.recv();

  if (packet == NULL) {
    return;
  }

  if (packet->id() == MASTER_PING_PACKET) {
    packets::Master::Ping *ping = (packets::Master::Ping *)packet;
    logger.info("Received ping packet");
    free(ping);

    packets::Slave::Pong *pong = new packets::Slave::Pong();
    connection.send(pong);
  } else if (packet->id() == MASTER_PROTOCOLTEST_PACKET) {
    packets::Master::ProtocolTest *ptest =
        (packets::Master::ProtocolTest *)packet;
    logger.info("Received protocol test packet");

    Serial.print("fieldA: ");
    Serial.println(ptest->fieldA);
    Serial.print("fieldB: ");
    Serial.println(ptest->fieldB);
    Serial.print("fieldC: ");
    Serial.println(ptest->fieldC);

    free(ptest);
  }

  return;

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
