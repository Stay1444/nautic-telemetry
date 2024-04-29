#pragma once

#include "FireTimer.h"
#include "metrics/MetricTask.h"
#include "radio/Connection.h"
#include <Arduino.h>

class Thermistor : public MetricTask {
public:
  Thermistor(uint8_t pin, uint8_t tag) {
    this->m_Pin = pin;
    m_Timer.begin(500);
  }

  float celsius();
  void tick(radio::Connection &radio) override;

private:
  uint8_t m_Pin = 0;
  uint8_t m_Tag = 0;
  FireTimer m_Timer;
};
