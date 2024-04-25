#pragma once

class Logger {
public:
  Logger(const char *name) { loggerName = name; }

  void info(const char *message);
  void warn(const char *message);
  void error(const char *message);

  const char *name() { return this->loggerName; }

private:
  const char *loggerName;
};
