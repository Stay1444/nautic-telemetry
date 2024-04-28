#pragma once

#include "Packet.h"
#include <Arduino.h>
#include <log/Logger.h>
#include <stdint.h>

namespace radio {

#define RADIO_PORT Serial1

class Connection {
public:
  Connection() {
    RADIO_PORT.begin(9600);
    m_Buffer = (uint8_t *)malloc(BUFFER_SIZE);
  }
  ~Connection() { free(m_Buffer); }

  static const size_t BUFFER_SIZE = 256;
  Packet *recv();
  void send(Packet *packet);

private:
  uint8_t *m_Buffer;
  size_t m_BufferLength = 0;
  Logger m_Logger = Logger("Radio");
};

} // namespace radio
