
#include "Packet.h"
#include "packets/Master.h"

using namespace radio;

Packet *Packet::deserialize(Cursor cursor, uint8_t packetId) {
  Packet *result;
  switch (packetId) {
  case 0:
    packets::Master::Ping *packet = new packets::Master::Ping();
    result = packet;
    break;
  }

  cursor.destroy();

  return result;
}
