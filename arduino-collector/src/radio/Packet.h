#pragma once

#include <stdint.h>

namespace radio {

class Packet {
public:
  virtual uint8_t id() const = 0;
  virtual ~Packet() = default;
  static Packet *deserialize(const uint8_t *data, uint32_t length);
};

} // namespace radio
