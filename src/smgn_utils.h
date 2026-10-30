#pragma once

static inline const char*
smgn_sanitize_string(const char* str)
{
  return str ? str : "<nullptr>";
}
