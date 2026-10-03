#pragma once
#include <SDL3/SDL_render.h>

SDL_Texture*
smgn_sdl_load_texture(SDL_Renderer* renderer, const char* image_path);
