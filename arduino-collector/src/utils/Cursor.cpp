#include "Cursor.h"
#include <Arduino.h>
#include <stdint.h>

Cursor::Cursor(const uint8_t *buffer, size_t length) {
  this->m_Buffer = buffer;
  this->m_Length = length;
}

bool Cursor::next(uint8_t &result) {
  if (this->m_Position >= this->m_Length) {
    return false;
  }
  result = this->m_Buffer[this->m_Position++];
  return true;
}

bool Cursor::next_u32(uint32_t &result) {
  uint8_t a = 0;
  uint8_t b = 0;
  uint8_t c = 0;
  uint8_t d = 0;

  if (!this->next(a))
    return false;
  if (!this->next(b))
    return false;
  if (!this->next(c))
    return false;
  if (!this->next(d))
    return false;

  result = ((uint32_t)a << 24) | ((uint32_t)b << 16) | ((uint32_t)c << 8) | d;
  return true;
}

void Cursor::skip(size_t count) {
  this->m_Position += count;
  if (this->m_Position > this->m_Length) {
    this->m_Position = this->m_Length;
  }
}
size_t Cursor::position() { return this->m_Position; }
size_t Cursor::length() { return this->m_Length; }
size_t Cursor::remaining() { return this->m_Length - this->m_Position; }

void Cursor::destroy() { free((void *)this->m_Buffer); }
