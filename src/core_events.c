#include "core_events.h"
#include "core.h"
#include <SDL3/SDL.h>

static void
dump_sdl_event(SDL_Event* sdl_event)
{
#if SDL_VERSION_ATLEAST(3, 4, 0)
  if (SDL_GetLogPriority(SDL_LOG_CATEGORY_APPLICATION) <=
      SDL_LOG_PRIORITY_VERBOSE) {
    char event_desc[128];

    SDL_GetEventDescription(sdl_event, event_desc, sizeof(event_desc));

    SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION,
                   "Received event: %s",
                   event_desc);
  }
#endif
}

void
smgn_events_process(smgn_t* smgn)
{
  SDL_Event sdl_event;
  while (SDL_PollEvent(&sdl_event)) {
    dump_sdl_event(&sdl_event);

    switch (sdl_event.type) {
      case SDL_EVENT_QUIT:
        smgn->should_quit = true;
        break;
    }
  }
}
