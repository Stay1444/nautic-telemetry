#pragma once

#include "utils/Task.h"
class Voltimeter : public Task {
public:
  Voltimeter(uint8_t pin, uint8_t tag, bool invert) {
    this->m_Pin = pin;
    this->m_Tag = tag;
    this->m_Invert = invert;
  }

  static float read(uint8_t pin);

  void flush(radio::Connection &radio) override;

private:
  uint8_t m_Pin = 0;
  uint8_t m_Tag = 0;
  bool m_Invert = false;
};
