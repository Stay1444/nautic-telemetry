#pragma once

#include <Arduino.h>

class Thermistor {
public:
  Thermistor(uint8_t pin) { this->m_Pin = pin; }

  float celsius();

private:
  uint8_t m_Pin = 0;
};
