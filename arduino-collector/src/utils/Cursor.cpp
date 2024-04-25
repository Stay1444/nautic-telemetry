#include "Cursor.h"

Cursor::Cursor(const uint8_t *buffer, size_t length) {
  this->m_Buffer = buffer;
  this->m_Length = length;
}

uint8_t Cursor::next() {
  if (this->m_Position + 1 >= this->m_Length) {
  }
  this->m_Buffer[0];
}
