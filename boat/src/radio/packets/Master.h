#include "radio/Packet.h"
#include "utils/Cursor.h"
#include <Arduino.h>
#include <stdint.h>

namespace radio::packets::Master {

#define MASTER_PING_PACKET 0

class Ping : public Packet {
public:
  uint8_t id() override { return MASTER_PING_PACKET; }
};

} // namespace radio::packets::Master
