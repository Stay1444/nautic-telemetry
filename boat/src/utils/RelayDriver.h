#pragma once

#include "FireTimer.h"
#include "radio/Connection.h"
#include "utils/Task.h"
#include <Arduino.h>

class RelayDriver : public Task {
public:
  RelayDriver(uint8_t relayPin, uint8_t voltimeterPin, float openThreshold,
              float choseThreshold, bool invert);

  void tick() override;
  void flush(radio::Connection &radio) override;

private:
  FireTimer m_Timer;
  uint8_t m_RelayPin;
  uint8_t m_VoltimeterPin;
  float m_OpenThreshold;
  float m_CloseThreshold;
  bool m_Invert;
  bool m_Open = false;
};
