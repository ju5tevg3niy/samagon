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
    'src/core.c',
    'src/core.h',
    'src/core_events.c',
    'src/core_events.h',
    'src/core_rendering.c',
    'src/core_rendering.h',
    'src/core_windowing.c',
    'src/core_windowing.h',
    dependencies: [
        sdl3,
    ],
)
