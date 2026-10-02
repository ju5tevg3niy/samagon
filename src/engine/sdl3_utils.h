#pragma once
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_log.h>

#ifndef NDEBUG
#define TRACE_FUNC                                                             \
  SDL_LogTrace(SDL_LOG_CATEGORY_APPLICATION, "Called %s", __func__);
#else
#define TRACE_FUNC
#endif

#define CALL_N_CHECK(SDL_FUNC, ...)                                            \
  do {                                                                         \
    bool ret = SDL_FUNC(__VA_ARGS__);                                          \
    if (!ret) {                                                                \
      SDL_LogError(SDL_LOG_CATEGORY_APPLICATION,                               \
                   "Function call " #SDL_FUNC " failed: %s",                   \
                   SDL_GetError());                                            \
      return false;                                                            \
    }                                                                          \
  } while (0);
