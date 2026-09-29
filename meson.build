project(
    'samagon',
    'c',
    meson_version: '>= 1.11.0',
    version: '0.1',
    default_options: [
        'warning_level=3',
        'c_std=c23',
        'b_ndebug=if-release',
    ],
)

sdl3 = dependency('sdl3')
flecs = dependency('flecs')

library(
    'samagon',
    'src/engine/sdl3.c',
    'src/engine/sdl3.h',
    'src/engine/sdl3_events.c',
    'src/engine/sdl3_events.h',
    'src/engine/sdl3_rendering.c',
    'src/engine/sdl3_rendering.h',
    'src/engine/sdl3_windowing.c',
    'src/engine/sdl3_windowing.h',
    dependencies: [
        sdl3,
    ],
)
