#include <stdio.h>
#include "helper_note.h"

int main() {
    
    // creating buffer
    NoteData note;
    note.idx = 0;
    note.is_full = false;

    // open file to write
    FILE *f = fopen("z_output.txt", "wb");

    helper_note_clear(&note);
    helper_note_add_text(&note, "Hello");
    helper_note_add_text_f(&note, ", %s!\n", "world");
    uint32_t arr[] = {1,2,3};
    helper_note_add_array_u32(&note, "numbers", arr, 3);
    helper_note_print(&note);

    helper_note_print_file(&note, f);

    return 0;
}
