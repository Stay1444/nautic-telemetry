#pragma once
#include <Arduino.h>
#include <stdint.h>

class Cursor {
public:
  Cursor(const uint8_t *buffer, size_t length);
  ~Cursor() = default;

  bool next(uint8_t &result);
  bool next_u32(uint32_t &result);
  void skip(size_t count);
  size_t position();
  size_t length();
  size_t remaining();
  void destroy();

private:
  size_t m_Length = 0;
  size_t m_Position = 0;
  const uint8_t *m_Buffer;
};
