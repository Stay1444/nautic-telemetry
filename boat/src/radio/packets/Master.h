#include "radio/Packet.h"
#include "utils/Cursor.h"
#include <Arduino.h>
#include <stdint.h>

namespace radio::packets::Master {

#define MASTER_START_SEND_WINDOW_PACKET 0

class StartSendWindow : public Packet {
public:
  uint8_t id() override { return MASTER_START_SEND_WINDOW_PACKET; }
};

} // namespace radio::packets::Master
