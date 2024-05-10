#include "log/Logger.h"
#include "metrics/Amperimeter.h"
#include "metrics/GPS.h"
#include "metrics/MetricTask.h"
#include "metrics/Thermistor.h"
#include "metrics/Voltimeter.h"
#include "radio/Connection.h"
#include "radio/packets/Master.h"
#include "radio/packets/Slave.h"
#include <Arduino.h>

static radio::Connection connection;
static Logger logger("main");

static MetricTask *tasks[4] = {new Thermistor(A1, 0), new GPS(8, 7),
                               new Amperimeter(A2, 0), new Voltimeter(A0, 0)};

void handlePacket(radio::Packet *packet) { Allocator::Free(packet); }

void setup() {
  Serial.begin(9600);
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
