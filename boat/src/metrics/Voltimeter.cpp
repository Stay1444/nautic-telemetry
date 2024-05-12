#include "metrics/Voltimeter.h"
#include "radio/Connection.h"
#include "radio/packets/Slave.h"

float Voltimeter::read(uint8_t pin) { return analogRead(pin) * 25.0 / 1024; }

void Voltimeter::flush(radio::Connection &radio) {
  auto packet = new radio::packets::Slave::Voltage();

  packet->tag = this->m_Tag;
  packet->voltage = Voltimeter::read(this->m_Pin);

  if (this->m_Invert) {
    packet->voltage *= -1;
  }

  radio.write(packet);
}
