#pragma once
#include <Arduino.h>
#include <stdint.h>

class Cursor {
public:
  Cursor(const uint8_t *buffer, size_t length);
  ~Cursor() = default;

  bool next(uint8_t &result);
  bool next(uint32_t &result);
  bool next(int32_t &result);
  bool next(uint64_t &result);
  bool next(bool &result);
  bool next(float &result);
  bool next(double &result);
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
