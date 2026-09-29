#include "sdl3_events.h"
#include "sdl3_utils.h"
#include <SDL3/SDL_events.h>
#include <SDL3/SDL_log.h>
#include <SDL3/SDL_version.h>

static void
dump_sdl_event(SDL_Event* sdl_event)
{
  if (SDL_GetLogPriority(SDL_LOG_CATEGORY_APPLICATION) <=
      SDL_LOG_PRIORITY_VERBOSE) {
#if SDL_VERSION_ATLEAST(3, 4, 0)
    char event_desc[128];
    SDL_GetEventDescription(sdl_event, event_desc, sizeof(event_desc));
#else
    char* event_desc = "<name ignored>";
    switch (sdl_event->type) {
      case SDL_EVENT_QUIT:
        event_desc = "SDL_EVENT_QUIT";
        break;
    }
#endif

    SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION,
                   "Received event: %s",
                   event_desc);
  }
}

bool
smgn_sdl_events_poll(SDL_Event* sdl_event)
{
  TRACE_FUNC

  bool has_event = SDL_PollEvent(sdl_event);

  if (has_event) {
    dump_sdl_event(sdl_event);
  }

  return has_event;
}
