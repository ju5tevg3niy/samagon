#include "core.h"
#include <SDL3/SDL.h>
#include <stdlib.h>

static void
sdl_init()
{
  SDL_InitFlags sdl_init_flags = SDL_INIT_VIDEO | SDL_INIT_EVENTS;

  SDL_Init(sdl_init_flags);
}

static void
sdl_quit()
{
  SDL_Quit();
}

smgn_t*
smgn_init()
{
  sdl_init();

  smgn_t* ret = calloc(1, sizeof(smgn_t));

  return ret;
}

void
smgn_quit(smgn_t* smgn)
{
  free(smgn);

  sdl_quit();
}

bool
smgn_get_should_quit(smgn_t* smgn)
{
  return smgn->should_quit;
}
