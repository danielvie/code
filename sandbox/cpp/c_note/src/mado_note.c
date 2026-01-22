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
void mado_note_add_text(NoteData *buffer, const char *format, ...) {
    // 1. Basic Safety Checks
    if (buffer == NULL || format == NULL || buffer->is_full) {
        return;
    }

    // 2. Format the input into a temporary string
    char temp_buffer[NOTE_TEMP_BUFFER_SIZE];
    va_list args;
    va_start(args, format);
    // vsnprintf returns the number of characters that *would* have been written
    int written = vsnprintf(temp_buffer, sizeof(temp_buffer), format, args);
    va_end(args);

    // Safety check: if vsnprintf failed or output was truncated
    if (written < 0) return;
    size_t text_length = (size_t)written;
    if (text_length >= sizeof(temp_buffer)) {
        text_length = sizeof(temp_buffer) - 1;
    }

    // 3. Calculate remaining space
    size_t remaining_space = (MADO_NOTE_FIXED_SIZE > buffer->idx) ? (MADO_NOTE_FIXED_SIZE - buffer->idx) : 0;

    // 4. Bounds Check
    if (text_length > remaining_space) {
        buffer->is_full = true;
        return;
    }

    // 5. Copy data (using memcpy is faster than a manual for-loop)
    memcpy(buffer->buffer + buffer->idx, temp_buffer, text_length);
    buffer->idx += text_length;

    // 6. Null termination (consistent with your previous logic)
    if (buffer->idx < MADO_NOTE_FIXED_SIZE) {
        buffer->buffer[buffer->idx] = '\0';
    }
}

// Add array
void mado_note_add_array_u32(NoteData *buffer, const char *name, const uint32_t *data, size_t data_len) {
    if (buffer == NULL || name == NULL) {
        return;
    }
    if (data_len == 0) {
        mado_note_add_text(buffer, "%s: [];\n", name);
        return;
    }

    mado_note_add_text(buffer, "%s: [", name);
    for (size_t i = 0; i < data_len; ++i) {
        if (i + 1 < data_len) {
            mado_note_add_text(buffer, "%u,", (unsigned)data[i]);
        } else {
            mado_note_add_text(buffer, "%u];\n", (unsigned)data[i]);
        }
    }
}

// Add array
void mado_note_add_array_i32(NoteData *buffer, const char *name, const int32_t *data, size_t data_len) {
    if (buffer == NULL || name == NULL) {
        return;
    }
    if (data_len == 0) {
        mado_note_add_text(buffer, "%s: [];\n", name);
        return;
    }

    mado_note_add_text(buffer, "%s: [", name);
    for (size_t i = 0; i < data_len; ++i) {
        if (i + 1 < data_len) {
            mado_note_add_text(buffer, "%d,", (int)data[i]);
        } else {
            mado_note_add_text(buffer, "%d];\n", (int)data[i]);
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

void mado_note_print_buffer(const NoteData *buffer) {
    char temp_log_buffer[MADO_NOTE_FIXED_SIZE + 128];
    char *ptr = temp_log_buffer;

    for (size_t i = 0; i < MADO_NOTE_FIXED_SIZE; ++i) {
        if (buffer->buffer[i] == '\0') {
            *ptr++ = '.';
        }
        else {
            *ptr++ = buffer->buffer[i];
        }
    }
    *ptr = '\0'; // null-terminator

    const float buffer_used = ((float)buffer->idx) / MADO_NOTE_FIXED_SIZE * 100.0;
    snprintf(ptr, sizeof(temp_log_buffer) - (ptr - temp_log_buffer),
             "\n\n[buffer size: %d, idx: %lld, used: %.1f%%]\n\n",
             MADO_NOTE_FIXED_SIZE, buffer->idx, buffer_used);

    printf("%s", temp_log_buffer);
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
