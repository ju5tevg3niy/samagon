#include "core_windowing.h"
#include <SDL3/SDL.h>

SDL_Window*
smgn_sdl_window_create()
{
  SDL_WindowFlags sdl_window_flags = 0;

  SDL_Window* window =
    SDL_CreateWindow("Window test", 900, 450, sdl_window_flags);

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
  SDL_DestroyWindow(window);
}
