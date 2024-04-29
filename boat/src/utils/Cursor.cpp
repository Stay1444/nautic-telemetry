#include "Cursor.h"
#include "utils/Allocator.h"
#include <Arduino.h>
#include <stdint.h>

Cursor::Cursor(const uint8_t *buffer, size_t length) {
  this->m_Buffer = buffer;
  this->m_Length = length;
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

void Cursor::destroy() {
  if (this->m_Buffer == NULL) {
    Serial.println("ERROR: Cursor tried to destroy a NULL buffer");
    return;
  }

  Allocator::Free((void *)this->m_Buffer);
  this->m_Buffer = NULL;
}

bool Cursor::next(uint8_t &result) {
  if (this->m_Position >= this->m_Length) {
    return false;
  }
  result = this->m_Buffer[this->m_Position++];
  return true;
}

bool Cursor::next(uint32_t &result) {
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

bool Cursor::next(int32_t &result) {
  uint32_t temp;
  if (!this->next(temp))
    return false;
  result = static_cast<int32_t>(temp);
  return true;
}

bool Cursor::next(uint64_t &result) {
  uint8_t a = 0, b = 0, c = 0, d = 0, e = 0, f = 0, g = 0, h = 0;
  if (!next(a) || !next(b) || !next(c) || !next(d) || !next(e) || !next(f) ||
      !next(g) || !next(h))
    return false;
  result = ((uint64_t)a << 56) | ((uint64_t)b << 48) | ((uint64_t)c << 40) |
           ((uint64_t)d << 32) | ((uint64_t)e << 24) | ((uint64_t)f << 16) |
           ((uint64_t)g << 8) | h;
  return true;
}

bool Cursor::next(double &result) {
  uint64_t temp;
  if (!this->next(temp))
    return false;
  result =
      *reinterpret_cast<double *>(&temp); // Interpret the uint64_t as a double
  return true;
}

bool Cursor::next(float &result) {
  uint32_t temp;
  if (!this->next(temp))
    return false;
  result = *reinterpret_cast<float *>(&temp);
  return true;
}

bool Cursor::next(bool &result) {
  uint8_t temp;
  if (!this->next(temp))
    return false;
  result = temp != 0;
  return true;
}
