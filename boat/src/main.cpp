#include "Gpio.h"
#include "log/Logger.h"
#include "metrics/Amperimeter.h"
#include "metrics/GPS.h"
#include "metrics/Thermistor.h"
#include "metrics/Voltimeter.h"
#include "radio/Connection.h"
#include "radio/packets/Master.h"
#include "radio/packets/Slave.h"
#include "utils/RelayDriver.h"
#include "utils/Task.h"
#include <Arduino.h>

static radio::Connection connection;
static Logger logger("main");

static Task *tasks[6] = {
    new Thermistor(GPIO_THERMISTOR_0, 0),
    new GPS(GPIO_GPS_RX, GPIO_GPS_TX),
    new Amperimeter(GPIO_AMPERIMETER_0, 0, true),
    new Amperimeter(GPIO_AMPERIMETER_1, 1, true),
    new Voltimeter(GPIO_VOLTIMETER_0, 0, false),
    new RelayDriver(GPIO_RELAY_0, GPIO_VOLTIMETER_0, 15.5, 13.0, false)};

void handlePacket(radio::Packet *packet) { Allocator::Free(packet); }

void setup() {
  Serial.begin(9600);
  connection.handler(handlePacket);
  logger.info("Ready");
}

void loop() {
  connection.tick();

  int tasksLength = sizeof(tasks) / sizeof(tasks[0]);

  bool sending = connection.isSending();

  for (int i = 0; i < tasksLength; ++i) {
    tasks[i]->tick();
    if (sending) {
      tasks[i]->flush(connection);
    }
  }
}
