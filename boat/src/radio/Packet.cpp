
#include "Packet.h"
#include "packets/Master.h"
#include "utils/Allocator.h"

using namespace radio;

Packet *Packet::deserialize(uint8_t *buffer, size_t bufferLength,
                            uint8_t packetId) {
  Packet *result = NULL;

  switch (packetId) {
  case MASTER_PING_PACKET:
    result = new packets::Master::Ping();
    break;
  }

  Allocator::Free(buffer);

  return result;
}
