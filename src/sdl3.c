#include <SDL3/SDL.h>

void smgn_sdl_init() {
    SDL_InitFlags sdl_init_flags = 0;

    SDL_Init(sdl_init_flags);
}

void smgn_sdl_quit() {
    SDL_Quit();
}
