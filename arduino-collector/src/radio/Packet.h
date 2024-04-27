#pragma once

#include "utils/Cursor.h"
#include "utils/Writer.h"
#include <stdint.h>

namespace radio {

struct PacketFrame {
  uint8_t id;
  Writer writer;
};

class Packet {
public:
  virtual ~Packet() = default;
  static Packet *deserialize(uint8_t *buffer, size_t bufferLength,
                             uint8_t packetId);
  virtual bool deserialize(Cursor &cursor) {
    Serial.println(
        "ERROR: TRYING TO DESERIALIZE A PACKET THAT ISN'T DESERIALIZABLE");
    return false;
  }

  virtual PacketFrame serialize() {
    Serial.println(
        "ERROR: TRYING TO SERIALIZE A PACKET THAT ISN'T SERIALIZABLE");
    return PacketFrame();
  }
  virtual uint8_t id();
};

} // namespace radio
