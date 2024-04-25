#pragma once
#include <Arduino.h>
#include <stdint.h>

class Cursor {
public:
  Cursor(const uint8_t *buffer, size_t length);
  ~Cursor();

  uint8_t next();
  uint32_t next_u32();
  void skip(uint32_t count);
  void skip(size_t count);
  size_t position();
  size_t length();

private:
  size_t m_Length = 0;
  size_t m_Position = 0;
  const uint8_t *m_Buffer;
};
