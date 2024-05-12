#pragma once
#include "radio/Connection.h"

class Task {
public:
  virtual ~Task() = default;
  virtual void tick() { return; };
  virtual void flush(radio::Connection &radio) { return; };
};
