#pragma once

#include "utils/Cursor.h"
#include <stdint.h>

namespace radio {

class Packet {
public:
  virtual uint8_t id() const = 0;
  virtual ~Packet() = default;
  static Packet *deserialize(Cursor cursor, uint8_t packetId);
};

} // namespace radio
