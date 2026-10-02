#include "sdl3_windowing.h"
#include "sdl3_utils.h"
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_log.h>
#include <SDL3/SDL_video.h>

SDL_Window*
smgn_sdl_window_create()
{
  TRACE_FUNC

  SDL_WindowFlags sdl_window_flags = 0;

  SDL_Window* window =
    SDL_CreateWindow("Window test", 640, 360, sdl_window_flags);

  if (!window) {
    SDL_LogError(SDL_LOG_CATEGORY_APPLICATION,
                 "Failed to create window: %s",
                 SDL_GetError());
  }

  return window;
}

void
smgn_sdl_window_nuke(SDL_Window* window)
{
  TRACE_FUNC

  SDL_DestroyWindow(window);
}
