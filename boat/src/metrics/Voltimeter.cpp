#include "metrics/Voltimeter.h"
#include "radio/Connection.h"
#include "radio/packets/Slave.h"

void Voltimeter::tick(radio::Connection &radio) {
  if (!this->m_Timer.fire())
    return;

  auto packet = new radio::packets::Slave::Voltage();

  packet->tag = this->m_Tag;
  packet->voltage = analogRead(this->m_Pin) * 25.0 / 1024;

  radio.queue(packet);
}
