#pragma once

#include "radio/Connection.h"
#include "utils/Task.h"
#include <Arduino.h>

class Thermistor : public Task {
public:
  Thermistor(uint8_t pin, uint8_t tag) : m_Logger("Thermistor") {
    this->m_Pin = pin;
  }

  static float celsius(uint8_t pin);
  void flush(radio::Connection &radio) override;

private:
  uint8_t m_Pin = 0;
  uint8_t m_Tag = 0;
  Logger m_Logger;
};
