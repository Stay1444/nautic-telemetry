
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
  case MASTER_PROTOCOLTEST_PACKET:
    Serial.println("deserializing protocoltest");
    result = new packets::Master::ProtocolTest();
    break;
  }

  free(buffer);

  Serial.println("Freed buffer");

  return result;
}
