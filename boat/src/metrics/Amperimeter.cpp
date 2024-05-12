#include "Amperimeter.h"
#include "radio/Connection.h"
#include "radio/packets/Slave.h"

void Amperimeter::flush(radio::Connection &radio) {
  auto packet = new radio::packets::Slave::Amperimeter();

  packet->tag = this->m_Tag;
  float amps = Amperimeter::read(this->m_Pin);
  if (this->m_Invert) {
    amps *= -1;
  }

  packet->value = amps;

  radio.write(packet);
}
