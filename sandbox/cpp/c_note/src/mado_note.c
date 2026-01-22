#include "mado_note.h"
#include <stdio.h>
#include <stdarg.h>
#include <string.h>

// Clear the buffer
void mado_note_clear(NoteData *buffer) {
    if (buffer == NULL) {
        return;
    }
    memset(buffer->buffer, 0, MADO_NOTE_FIXED_SIZE);
    buffer->idx = 0;
    buffer->is_full = false;
}

// Add a text
void mado_note_add_text(NoteData *buffer, const char *text_to_add) {
    if (buffer == NULL || text_to_add == NULL) {
        return;
    }
    if (buffer->is_full) {
        return;
    }

    size_t text_length = strlen(text_to_add);

    /* remaining_space is total bytes left (including space for a trailing '\0' if possible) */
    size_t remaining_space = (MADO_NOTE_FIXED_SIZE > buffer->idx) ? (MADO_NOTE_FIXED_SIZE - buffer->idx) : 0;

    if (text_length > remaining_space) {
        /* Not enough room to copy the text; mark full and do not write partial content. */
        buffer->is_full = true;
        return;
    }

    /* Copy the text */
    memcpy(buffer->buffer + buffer->idx, text_to_add, text_length);
    buffer->idx += text_length;

    /* Add a null terminator if there's space (keeps behavior similar to the C++ version) */
    if (buffer->idx < MADO_NOTE_FIXED_SIZE) {
        buffer->buffer[buffer->idx] = '\0';
    }
}

// Add formatted text
void mado_note_add_text_f(NoteData *buffer, const char *format, ...) {
    if (buffer == NULL || format == NULL) {
        return;
    }
    if (buffer->is_full) {
        /* Optionally, you can print a diagnostic: */
        /* printf("Buffer is full. Cannot add new text.\n"); */
        return;
    }

    char temp_buffer[NOTE_TEMP_BUFFER_SIZE];
    va_list args;
    va_start(args, format);
    vsnprintf(temp_buffer, sizeof(temp_buffer), format, args);
    va_end(args);

    mado_note_add_text(buffer, temp_buffer);
}

// Add array
void mado_note_add_array_u32(NoteData *buffer, const char *name, const uint32_t *data, size_t data_len) {
    if (buffer == NULL || name == NULL) {
        return;
    }
    if (data_len == 0) {
        mado_note_add_text_f(buffer, "%s: [];\n", name);
        return;
    }

    mado_note_add_text_f(buffer, "%s: [", name);
    for (size_t i = 0; i < data_len; ++i) {
        if (i + 1 < data_len) {
            mado_note_add_text_f(buffer, "%u,", (unsigned)data[i]);
        } else {
            mado_note_add_text_f(buffer, "%u];\n", (unsigned)data[i]);
        }
    }
}

// Add array
void mado_note_add_array_i32(NoteData *buffer, const char *name, const int32_t *data, size_t data_len) {
    if (buffer == NULL || name == NULL) {
        return;
    }
    if (data_len == 0) {
        mado_note_add_text_f(buffer, "%s: [];\n", name);
        return;
    }

    mado_note_add_text_f(buffer, "%s: [", name);
    for (size_t i = 0; i < data_len; ++i) {
        if (i + 1 < data_len) {
            mado_note_add_text_f(buffer, "%d,", (int)data[i]);
        } else {
            mado_note_add_text_f(buffer, "%d];\n", (int)data[i]);
        }
    }
}

// Print current text in the buffer
void mado_note_print(const NoteData *buffer) {
    if (buffer == NULL) {
        return;
    }

    if (buffer->idx < MADO_NOTE_FIXED_SIZE && buffer->buffer[buffer->idx] == '\0') {
        printf("%s", buffer->buffer);
    } else {
        fwrite(buffer->buffer, 1, buffer->idx, stdout);
    }
}

// Print current text to a file
bool mado_note_print_file(const NoteData *nb, FILE *f) {
    if (nb == NULL || f == NULL) {
        return false;
    }

    size_t to_write = nb->idx;
    if (to_write == 0) {
        return true;
    }

    size_t written = fwrite(nb->buffer, 1, to_write, f);
    if (written != to_write) {
        return false;
    }

    if (fflush(f) != 0) {
        return false;
    }

    return true;
}
