#pragma once
#include "SoftwareSerial.h"
#include "TinyGPS++.h"
#include "log/Logger.h"
#include "radio/Connection.h"
#include "utils/Task.h"
#include <Arduino.h>

class GPS : public Task {
public:
  GPS(uint8_t rx, uint8_t tx) : m_Serial(rx, tx), m_Logger("GPS") {
    this->m_Serial.begin(9600);
  }

  void flush(radio::Connection &radio) override;

private:
  SoftwareSerial m_Serial;
  TinyGPSPlus m_Gps;

  Logger m_Logger;
};
