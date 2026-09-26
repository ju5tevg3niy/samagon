#pragma once

typedef struct
{
  bool should_quit;
} smgn_t;

smgn_t*
smgn_init();

void
smgn_quit(smgn_t* smgn);

bool
smgn_get_should_quit(smgn_t* smgn);
