#include "log/Logger.h"
#include "radio/Connection.h"
#include "radio/packets/Master.h"
#include "radio/packets/Slave.h"
#include <Arduino.h>

static radio::Connection connection;
static Logger logger("main");

using namespace radio;

void setup() {
  Serial.begin(9600); // Serial port to computer
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
  }

  return;
}
