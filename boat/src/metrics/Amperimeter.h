#pragma once

#include "radio/Connection.h"
#include "utils/Task.h"
#include <Arduino.h>

class Amperimeter : public Task {
public:
  static float read(uint8_t pin, int samples = 100) {
    float volts;
    float current = 0;
    for (int i = 0; i < samples; i++) {
      volts = analogRead(pin) * (5.0 / 1023.0);
      current = current + (volts - 2.5) / 0.066;
    }
    current = current / samples;
    return current;
  }

  Amperimeter(uint8_t pin, uint8_t tag, bool invert) {
    this->m_Pin = pin;
    this->m_Tag = tag;
    this->m_Invert = invert;
  }

  void flush(radio::Connection &radio) override;

private:
  uint8_t m_Pin = 0;
  uint8_t m_Tag = 0;
  bool m_Invert = false;
};
