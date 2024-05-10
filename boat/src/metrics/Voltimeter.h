#pragma once

#include "FireTimer.h"
#include "metrics/MetricTask.h"
class Voltimeter : public MetricTask {
public:
  Voltimeter(uint8_t pin, uint8_t tag) {
    this->m_Pin = pin;
    this->m_Timer.begin(1000);
    this->m_Tag = tag;
  }

  void tick(radio::Connection &radio) override;

private:
  uint8_t m_Pin = 0;
  uint8_t m_Tag = 0;
  FireTimer m_Timer;
};
