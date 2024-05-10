#pragma once

#include "FireTimer.h"
#include "metrics/MetricTask.h"
class Voltimeter : public MetricTask {
public:
  Voltimeter(uint8_t pin, uint8_t tag, bool invert) {
    this->m_Pin = pin;
    this->m_Timer.begin(1000);
    this->m_Tag = tag;
    this->m_Invert = invert;
  }

  static float read(uint8_t pin);

  void tick(radio::Connection &radio) override;

private:
  uint8_t m_Pin = 0;
  uint8_t m_Tag = 0;
  bool m_Invert = false;
  FireTimer m_Timer;
};
