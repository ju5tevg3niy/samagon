#include "sdl3.h"
#include "sdl3_utils.h"
#include <SDL3/SDL_init.h>
#include <SDL3/SDL_log.h>

bool
smgn_sdl_init()
{
  TRACE_FUNC

  SDL_InitFlags sdl_init_flags = 0;
  sdl_init_flags |= SDL_INIT_EVENTS;
  sdl_init_flags |= SDL_INIT_VIDEO;

  CALL_N_CHECK(SDL_Init, sdl_init_flags);

  return true;
}

void
smgn_sdl_quit()
{
  TRACE_FUNC

  SDL_Quit();
}
