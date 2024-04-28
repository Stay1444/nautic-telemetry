
#include "Packet.h"
#include "packets/Master.h"

using namespace radio;

Packet *Packet::deserialize(uint8_t *buffer, size_t bufferLength,
                            uint8_t packetId) {
  Packet *result = NULL;

  Serial.print("Deserializing packet with id ");
  Serial.println(packetId);

  switch (packetId) {
  case MASTER_PING_PACKET:
    result = new packets::Master::Ping();
    break;
  }

  free(buffer);

  Serial.println("Freed buffer");

  return result;
}
