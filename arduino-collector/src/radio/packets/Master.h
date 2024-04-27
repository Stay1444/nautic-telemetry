#include "radio/Packet.h"
#include "utils/Cursor.h"
#include <Arduino.h>
#include <stdint.h>

namespace radio::packets::Master {

#define MASTER_PING_PACKET 0
#define MASTER_PROTOCOLTEST_PACKET 1

class Ping : public Packet {
public:
  uint8_t id() override { return MASTER_PING_PACKET; }
};

class ProtocolTest : public Packet {
public:
  uint8_t id() override { return MASTER_PROTOCOLTEST_PACKET; }
  float fieldA;
  uint32_t fieldB;
  int32_t fieldC;

  bool deserialize(Cursor &cursor) override {
    Serial.println("hello");
    return true;

    if (!cursor.next(this->fieldA))
      return false;

    if (!cursor.next(this->fieldB))
      return false;

    if (!cursor.next(this->fieldC))
      return false;

    return true;
  }
};

} // namespace radio::packets::Master
