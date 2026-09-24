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

rust = import('rust')

cargo_ws = rust.workspace()

pkg = cargo_ws.package()

samagon_rust_lib = pkg.library()

exe = executable(
    'samagon',
    'src/main.c',
    link_with: samagon_rust_lib,
)

test('basic', exe)
