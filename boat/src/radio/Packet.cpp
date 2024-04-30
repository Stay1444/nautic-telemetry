
#include "Packet.h"
#include "packets/Master.h"
#include "utils/Allocator.h"

using namespace radio;

Packet *Packet::deserialize(uint8_t *buffer, size_t bufferLength,
                            uint8_t packetId) {
  Packet *result = NULL;

  Cursor cursor(buffer, bufferLength);
  bool success = true;

  switch (packetId) {
  case MASTER_PING_PACKET:
    result = new packets::Master::Ping();
    success = result->deserialize(cursor);
    break;
  }

  Allocator::Free(buffer);

  if (!success) {
    Serial.print("ERROR deserializing packet. Buffer was not big "
                 "enough? Buffer length: ");
    Serial.print(bufferLength);
    Serial.print(" packet id: ");
    Serial.println(packetId);
    if (result != NULL) {
      free(result);
    }
    result = NULL;
  }

  return result;
}
