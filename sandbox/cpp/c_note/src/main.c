#include <corecrt.h>
#include <stdio.h>
#include "mado_note.h"

int main() {

    // creating buffer
    NoteData note;
    note.idx = 0;
    note.is_full = false;

    // open file to write
    FILE *f;
    errno_t err = fopen_s(&f, "z_output.txt", "wb");

    mado_note_clear(&note);
    mado_note_add_text(&note, "Hello");
    mado_note_add_text_f(&note, ", %s!\n", "world");
    uint32_t arr[] = {1,2,3};
    mado_note_print(&note);

    mado_note_print_file(&note, f);

    return 0;
}
