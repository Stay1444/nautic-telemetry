#include "log/Logger.h"
#include "radio/Connection.h"
#include <Arduino.h>

void setup() {
  Serial.begin(9600);  // Serial port to computer
  Serial1.begin(9600); // Serial port to HC1
}

static radio::Connection conn;
static Logger logger = Logger("main");
void loop() { conn.tick(); }
