#include "Amperimeter.h"
#include "radio/Connection.h"
#include "radio/packets/Slave.h"

void Amperimeter::tick(radio::Connection &radio) {
  if (!this->m_Timer.fire())
    return;

  auto packet = new radio::packets::Slave::Amperimeter();

  packet->tag = this->m_Tag;
  packet->value = Amperimeter::read(this->m_Pin);

  radio.queue(packet);
}
