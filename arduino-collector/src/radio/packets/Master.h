#include "radio/Packet.h"
#include <Arduino.h>
#include <stdint.h>

namespace radio::packets::Master {

#define MASTER_PING_PACKET 0

class Ping : public Packet {
public:
  static constexpr uint8_t Id = 0;
  uint8_t id() override { return Id; }
};

} // namespace radio::packets::Master
