#pragma once
#include <SDL3/SDL_log.h>

#ifndef NDEBUG
#define TRACE_FUNC                                                             \
  SDL_LogTrace(SDL_LOG_CATEGORY_APPLICATION, "Called %s", __func__);
#else
#define TRACE_FUNC
#endif
