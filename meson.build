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

library(
    'samagon',
    'src/lib.c',
    'src/lib.h',
    'src/sdl3.c',
    dependencies: [
        sdl3,
    ],
)
