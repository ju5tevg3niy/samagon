#pragma once
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>

SDL_Renderer*
smgn_sdl_renderer_create(SDL_Window* window);

void
smgn_sdl_renderer_nuke(SDL_Renderer* renderer);

void
smgn_sdl_renderer_begin(SDL_Renderer* renderer,
                        uint8_t r,
                        uint8_t g,
                        uint8_t b);

void
smgn_sdl_renderer_end(SDL_Renderer* renderer);

void
smgn_sdl_renderer_render_texture(SDL_Renderer* renderer);
