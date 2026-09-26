#pragma once
#include <SDL3/SDL.h>

SDL_Window*
smgn_sdl_window_create();

void
smgn_sdl_window_nuke(SDL_Window* window);
