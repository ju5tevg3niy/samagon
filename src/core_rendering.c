#include "core_rendering.h"
#include <SDL3/SDL.h>

SDL_Renderer*
smgn_sdl_renderer_create(SDL_Window* window)
{
  SDL_Renderer* renderer = SDL_CreateRenderer(window, NULL);

  if (!renderer) {
    SDL_LogError(SDL_LOG_CATEGORY_APPLICATION,
                 "Failed to create renderer for window: %s",
                 SDL_GetError());
  }

  return renderer;
}

void
smgn_sdl_renderer_nuke(SDL_Renderer* renderer)
{
  SDL_DestroyRenderer(renderer);
}

void
smgn_sdl_renderer_render(SDL_Renderer* renderer,
                         uint8_t r,
                         uint8_t g,
                         uint8_t b)
{
  SDL_SetRenderDrawColor(renderer, r, g, b, SDL_ALPHA_OPAQUE);
  SDL_RenderClear(renderer);
  SDL_RenderPresent(renderer);
}
