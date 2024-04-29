#pragma once
#include "SoftwareSerial.h"
#include "TinyGPS++.h"
#include "metrics/MetricTask.h"
#include "radio/Connection.h"
#include <Arduino.h>

class GPS : public MetricTask {
public:
  GPS(uint8_t rx, uint8_t tx) : m_Serial(rx, tx) { this->m_Serial.begin(9600); }

  void tick(radio::Connection &radio) override;

private:
  SoftwareSerial m_Serial;
  TinyGPSPlus m_Gps;
};
