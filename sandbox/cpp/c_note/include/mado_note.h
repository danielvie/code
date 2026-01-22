#ifndef MADO_NOTE_H
#define MADO_NOTE_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

#define NOTE_TEMP_BUFFER_SIZE 256
#define MADO_NOTE_FIXED_SIZE 4096

typedef struct {
    char buffer[MADO_NOTE_FIXED_SIZE];     /* pointer to the pre-allocated buffer memory */
    size_t idx;       /* current write index (next char position) */
    bool is_full;     /* true when further writes are blocked */
} NoteData;

/* Function declarations */
void mado_note_clear(NoteData *buffer);

void mado_note_add_text(NoteData *buffer, const char *format, ...);

void mado_note_add_array_u32(NoteData *buffer, const char *name, const uint32_t *data, size_t data_len);
void mado_note_add_array_i32(NoteData *buffer, const char *name, const int32_t *data, size_t data_len);

void mado_note_print(const NoteData *buffer);
void mado_note_print_buffer(const NoteData *buffer);
bool mado_note_print_file(const NoteData *nb, FILE *f);

#endif /* MADO_NOTE_H */
