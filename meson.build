project(
    'samagon',
    'c',
    meson_version: '>= 1.3.0',
    version: '0.1',
    default_options: [
        'warning_level=3',
        'c_std=c23',
    ],
)

exe = executable('samagon', 'src/main.c')

test('basic', exe)
