#pragma once

#include "FireTimer.h"
#include "metrics/MetricTask.h"
#include "radio/Connection.h"
#include <Arduino.h>

class RelayDriver : public MetricTask {
public:
  RelayDriver(uint8_t relayPin, uint8_t thermistorPin, float threshold,
              bool invert);

  void tick(radio::Connection &radio);

private:
  FireTimer m_Timer;
  uint8_t m_RelayPin;
  uint8_t m_ThermistorPin;
  float m_Threshold;
  bool m_Invert;
};
