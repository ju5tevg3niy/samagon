#include "sdl3_events.h"
#include "sdl3_utils.h"
#include "smgn_utils.h"
#include <SDL3/SDL_events.h>
#include <SDL3/SDL_log.h>
#include <SDL3/SDL_version.h>
#include <stdio.h>

static void
dump_sdl_event(SDL_Event* sdl_event)
{
  if (SDL_GetLogPriority(SDL_LOG_CATEGORY_APPLICATION) <=
      SDL_LOG_PRIORITY_VERBOSE) {
    char event_desc_buf[256];
    char* event_desc = nullptr;

#if SDL_VERSION_ATLEAST(3, 4, 0)
    SDL_GetEventDescription(sdl_event, event_desc_buf, sizeof(event_desc_buf));

    event_desc = event_desc_buf;
#else
    // NOLINTNEXTLINE
    snprintf(event_desc_buf,
             sizeof(event_desc_buf),
             "<name ingored>, event type %d",
             sdl_event->type);

    event_desc = event_desc_buf;

    switch (sdl_event->type) {
      case SDL_EVENT_QUIT:
        event_desc = "SDL_EVENT_QUIT";
        break;
      case SDL_EVENT_KEYBOARD_ADDED:
        event_desc = "SDL_EVENT_KEYBOARD_ADDED";
        break;
      case SDL_EVENT_KEYBOARD_REMOVED:
        event_desc = "SDL_EVENT_KEYBOARD_REMOVED";
        break;
      case SDL_EVENT_WINDOW_MOUSE_ENTER:
        event_desc = "SDL_EVENT_WINDOW_MOUSE_ENTER";
        break;
      case SDL_EVENT_WINDOW_MOUSE_LEAVE:
        event_desc = "SDL_EVENT_WINDOW_MOUSE_LEAVE";
        break;
      default:
        break;
    }
#endif

    SDL_LogVerbose(SDL_LOG_CATEGORY_APPLICATION,
                   "Received event: %s",
                   smgn_sanitize_string(event_desc));
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
