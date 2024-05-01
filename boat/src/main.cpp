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

void handlePacket(radio::Packet *packet) { free(packet); }

void setup() {
  Serial.begin(9600); // Serial port to computer
  connection.handler(handlePacket);
  logger.info("Ready");
}

void loop() {
  connection.tick();

  int tasksLength = sizeof(tasks) / sizeof(tasks[0]);

  for (int i = 0; i < tasksLength; ++i) {
    tasks[i]->tick(connection);
  }
}
