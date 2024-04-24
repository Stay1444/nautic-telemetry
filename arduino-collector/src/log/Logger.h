#pragma once

class Logger {
public:
  Logger(const char *name) { loggerName = name; }

  void info(const char *fmt, ...);
  void warn(const char *fmt, ...);
  void error(const char *fmt, ...);

  const char *name() { return this->loggerName; }

private:
  const char *loggerName;
};
