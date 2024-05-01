#pragma once

#include <Arduino.h>

class Vector {
public:
  Vector();
  ~Vector();

  void push(void *entry);
  void *pop();

  size_t length();
  size_t capacity();

  void clear();

private:
  size_t m_Length = 0;
  size_t m_Capacity = 0;
  void **m_Slice;

  void extend();
};
