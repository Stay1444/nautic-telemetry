#pragma once

#include "Packet.h"
#include <Arduino.h>
#include <log/Logger.h>
#include <stdint.h>

namespace radio {

class Connection {
public:
  Connection() { m_Buffer = (uint8_t *)malloc(BUFFER_SIZE); }
  ~Connection() { free(m_Buffer); }

  static const size_t BUFFER_SIZE = 1024;
  Packet *recv();

private:
  uint8_t *m_Buffer;
  size_t m_BufferLength = 0;
  Logger m_Logger = Logger("Radio");
};

} // namespace radio
