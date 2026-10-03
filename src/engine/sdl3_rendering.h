#pragma once
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>

SDL_Renderer*
smgn_sdl_renderer_create(SDL_Window* window);

void
smgn_sdl_renderer_nuke(SDL_Renderer* renderer);

bool
smgn_sdl_renderer_start(SDL_Renderer* renderer,
                        uint8_t r,
                        uint8_t g,
                        uint8_t b);

bool
smgn_sdl_renderer_finish(SDL_Renderer* renderer);

bool
smgn_sdl_renderer_render_texture(SDL_Renderer* renderer, SDL_Texture* texture);

void
smgn_sdl_renderer_nuke_texture(SDL_Texture* texture);
