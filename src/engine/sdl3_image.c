#include "sdl3_image.h"
#include "sdl3_utils.h"
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_log.h>
#include <SDL3/SDL_render.h>
#include <SDL3_image/SDL_image.h>

SDL_Texture*
smgn_sdl_load_texture(SDL_Renderer* renderer, const char* image_path)
{
  TRACE_FUNC

  SDL_Texture* texture = IMG_LoadTexture(renderer, image_path);

  if (!texture) {
    SDL_LogError(SDL_LOG_CATEGORY_APPLICATION,
                 "Failed to load image: %s",
                 SDL_GetError());
  }

  return texture;
}
