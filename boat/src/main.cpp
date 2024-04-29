#include "log/Logger.h"
#include "metrics/GPS.h"
#include "metrics/MetricTask.h"
#include "metrics/Thermistor.h"
#include "radio/Connection.h"
#include "radio/packets/Master.h"
#include "radio/packets/Slave.h"
#include <Arduino.h>

static radio::Connection connection;
static Logger logger("main");

static MetricTask *tasks[2] = {new Thermistor(A1, 0), new GPS(8, 7)};

using namespace radio;

void setup() {
  Serial.begin(9600); // Serial port to computer

  delay(1000);
  logger.info("Ready");
}

void loop() {
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

  int tasksLength = sizeof(tasks) / sizeof(tasks[0]);

  for (int i = 0; i < tasksLength; ++i) {
    tasks[i]->tick(connection);
  }
}
