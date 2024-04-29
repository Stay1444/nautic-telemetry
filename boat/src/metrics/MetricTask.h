#pragma once
#include "radio/Connection.h"

class MetricTask {
public:
  virtual ~MetricTask() = default;
  virtual void tick(radio::Connection &radio);
};
