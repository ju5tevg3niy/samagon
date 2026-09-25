#include <stdlib.h>
#include <SDL3/SDL.h>

void smgn_sdl_init() {
    SDL_InitFlags sdl_init_flags = SDL_INIT_VIDEO;

    SDL_Init(sdl_init_flags);
}

void smgn_sdl_quit() {
    SDL_Quit();
}

typedef struct
{
    bool should_quit;
} smgn_t;

smgn_t* smgn_init() {
    return calloc(1, sizeof(smgn_t));
}

void smgn_quit(smgn_t* smgn) {
    free(smgn);
}

void smgn_process_events(smgn_t* smgn) {
    SDL_Event sdl_event;
    while(SDL_PollEvent(&sdl_event)) {
        switch (sdl_event.type) {
        case SDL_EVENT_QUIT:
            smgn->should_quit = true;
            break;
        }
    }
}

bool smgn_get_should_quit(smgn_t* smgn) {
    return smgn->should_quit;
}

SDL_Window* smgn_sdl_get_window() {
    SDL_WindowFlags sdl_window_flags = 0;

    SDL_Window* window = SDL_CreateWindow("Window test", 900, 450, sdl_window_flags);

    if (!window) {
        SDL_Log("Failed to create window: %s", SDL_GetError());
    }

    return window;
}

void smgn_sdl_nuke_window(SDL_Window* window) {
    SDL_DestroyWindow(window);
}

SDL_Renderer* smgn_sdl_get_renderer(SDL_Window* window) {
    SDL_Renderer* renderer = SDL_CreateRenderer(window, NULL);

    if (!renderer) {
        SDL_Log("Failed to create renderer for window: %s", SDL_GetError());
    }

    return renderer;
}

void smgn_sdl_nuke_renderer(SDL_Renderer* renderer) {
    SDL_DestroyRenderer(renderer);
}

void smgn_sdl_render(SDL_Renderer* renderer, uint8_t r,uint8_t g,uint8_t b){
    SDL_SetRenderDrawColor(renderer, r,g,b,SDL_ALPHA_OPAQUE);
    SDL_RenderClear(renderer);
    SDL_RenderPresent(renderer);
}
