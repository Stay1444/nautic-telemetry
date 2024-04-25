#pragma once

#include <Arduino.h>
#include <log/Logger.h>
#include <stdint.h>

namespace radio {

class Connection {
public:
  Connection() { buffer = (uint8_t *)malloc(BUFFER_SIZE); }
  ~Connection() { free(buffer); }

  static const size_t BUFFER_SIZE = 1024;
  void tick();

private:
  uint8_t *buffer;
  size_t bufferLength = 0;
  Logger logger = Logger("Radio");
};

} // namespace radio
