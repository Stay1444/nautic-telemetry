#include "FireTimer.h"
#include "SoftwareSerial.h"
#include "TinyGPS++.h"
#include "Utils.h"
#include "log/Logger.h"
#include "metrics/Thermistor.h"
#include "radio/Connection.h"
#include "radio/packets/Master.h"
#include "radio/packets/Slave.h"
#include <Arduino.h>

static radio::Connection connection;
static Logger logger("main");
static SoftwareSerial gpsSerial(8, 7);
static TinyGPSPlus gps;
static Thermistor therm(A1);
static FireTimer thermTimer;

static FireTimer voltageTimer;

using namespace radio;

void setup() {
  Serial.begin(9600); // Serial port to computer
  gpsSerial.begin(9600);
  thermTimer.begin(500);
  voltageTimer.begin(500);

  delay(1000);
  logger.info("Ready");

  pinMode(4, OUTPUT);
  digitalWrite(4, HIGH);
}

void loop() {
  digitalWrite(4, LOW);
  RADIO_PORT.println("AT");
  while (RADIO_PORT.available()) {
    Serial.print((char)RADIO_PORT.read());
  }
  delay(1000);
  return;
  radio::Packet *packet = NULL; // connection.recv();

  if (packet != NULL) {
    if (packet->id() == MASTER_PING_PACKET) {
      packets::Master::Ping *ping = (packets::Master::Ping *)packet;

      logger.info("Received ping packet");
      free(ping);

      packets::Slave::Pong *pong = new packets::Slave::Pong();
      connection.send(pong);
    }
  }

  if (thermTimer.fire()) {
    auto packet = new packets::Slave::Temperature();

    packet->tag = 0;
    packet->temperature = therm.celsius();

    connection.send(packet);
  }

  if (voltageTimer.fire()) {
    auto packet = new packets::Slave::Voltage();

    packet->tag = 0;
    packet->voltage = analogRead(A0) * 25.0 / 1024;

    connection.send(packet);
  }

  while (gpsSerial.available()) {
    gps.encode(gpsSerial.read());
  }

  if ((gps.satellites.isUpdated() && gps.satellites.isValid()) ||
      gps.location.isUpdated() || gps.speed.isUpdated()) {

    return;
    auto packet = new packets::Slave::GPS();

    packet->satellites = (uint8_t)gps.satellites.value();
    packet->lat = gps.location.lat();
    packet->lon = gps.location.lng();
    packet->mps = (float)gps.speed.mps();

    connection.send(packet);
  }
}
