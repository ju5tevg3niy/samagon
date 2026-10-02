#include "sdl3_rendering.h"
#include "sdl3_utils.h"
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_log.h>
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>

SDL_Renderer*
smgn_sdl_renderer_create(SDL_Window* window)
{
  TRACE_FUNC

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
  TRACE_FUNC

  SDL_DestroyRenderer(renderer);
}

bool
smgn_sdl_renderer_start(SDL_Renderer* renderer, uint8_t r, uint8_t g, uint8_t b)
{
  TRACE_FUNC

  bool ret;

  ret = SDL_SetRenderDrawColor(renderer, r, g, b, SDL_ALPHA_OPAQUE);

  if (!ret) {
    SDL_LogError(SDL_LOG_CATEGORY_APPLICATION,
                 "Failed to set render draw color: %s",
                 SDL_GetError());

    return false;
  }

  ret = SDL_RenderClear(renderer);

  if (!ret) {
    SDL_LogError(SDL_LOG_CATEGORY_APPLICATION,
                 "Failed to clear renderer: %s",
                 SDL_GetError());

    return false;
  }

  return true;
}

bool
smgn_sdl_renderer_finish(SDL_Renderer* renderer)
{
  TRACE_FUNC

  bool ret = SDL_RenderPresent(renderer);

  if (!ret) {
    SDL_LogError(SDL_LOG_CATEGORY_ERROR,
                 "Failed to present renderer: %s",
                 SDL_GetError());

    return false;
  }

  return true;
}
