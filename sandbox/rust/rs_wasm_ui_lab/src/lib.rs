//! THROWAWAY PROTOTYPE: test whether Rust/Wasm can own a browser UI while the
//! DOM is limited to a canvas and a hidden textarea used for text/IME input.

#![cfg_attr(all(not(target_arch = "wasm32"), not(test)), allow(dead_code))]

use regex::RegexBuilder;
use unicode_segmentation::UnicodeSegmentation;

const ITEM_COUNT: usize = 10_000;
const ROW_HEIGHT: f64 = 34.0;
#[cfg(target_arch = "wasm32")]
const DEFAULT_TEXT: &str = "Rust owns this text buffer.\nSelect, copy, paste, and undo normally.\nUnicode: café · 日本語 · العربية · 🦀";

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    fn contains(self, x: f64, y: f64) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

fn clamp_scroll_for(item_count: usize, scroll: f64, viewport_height: f64) -> f64 {
    let content_height = item_count as f64 * ROW_HEIGHT;
    scroll.clamp(0.0, (content_height - viewport_height).max(0.0))
}

fn visible_range_for(
    item_count: usize,
    scroll: f64,
    viewport_height: f64,
) -> std::ops::Range<usize> {
    let start = (scroll / ROW_HEIGHT).floor() as usize;
    let visible_count = (viewport_height / ROW_HEIGHT).ceil() as usize + 1;
    start.min(item_count)..(start + visible_count).min(item_count)
}

fn row_status(row: usize) -> &'static str {
    match row % 3 {
        0 => "Ready",
        1 => "Review",
        _ => "Blocked",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Focus {
    Action,
    Toggle,
    Text,
    List,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorkbenchMode {
    DocumentData,
    Diff,
    Transcript,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiffPane {
    Files,
    Content,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TranscriptPane {
    Speakers,
    Segments,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TranscriptSegment {
    seconds: usize,
    speaker: usize,
    text: String,
}

fn generate_transcript(count: usize) -> Vec<TranscriptSegment> {
    const PHRASES: [&str; 6] = [
        "We need to verify the browser input boundary.",
        "The canvas owns rendering and interaction state.",
        "Search should move to the matching segment.",
        "Playback follows the active timestamp when enabled.",
        "Selection can include several transcript segments.",
        "Accessibility mirrors the active segment and controls.",
    ];
    (0..count)
        .map(|index| TranscriptSegment {
            seconds: index * 4,
            speaker: index % 4,
            text: format!("{} Segment {}.", PHRASES[index % PHRASES.len()], index + 1),
        })
        .collect()
}

fn transcript_matches(segments: &[TranscriptSegment], query: &str) -> Vec<usize> {
    let query = query.trim();
    if query.is_empty() {
        return Vec::new();
    }
    segments
        .iter()
        .enumerate()
        .filter_map(|(index, segment)| fuzzy_score(&segment.text, query).map(|_| index))
        .collect()
}

fn format_timestamp(seconds: usize) -> String {
    if seconds >= 3_600 {
        format!(
            "{}:{:02}:{:02}",
            seconds / 3_600,
            (seconds / 60) % 60,
            seconds % 60
        )
    } else {
        format!("{:02}:{:02}", seconds / 60, seconds % 60)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiffKind {
    Unchanged,
    Added,
    Removed,
    Modified,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DiffLine {
    old_number: Option<usize>,
    new_number: Option<usize>,
    old_text: String,
    new_text: String,
    kind: DiffKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiffDisplayRow {
    Line(usize),
    Fold { start: usize, count: usize },
}

fn generate_diff(file: usize, line_count: usize) -> Vec<DiffLine> {
    let mut old_number = 1;
    let mut new_number = 1;
    (0..line_count)
        .map(|index| {
            let kind = if index % (83 + file * 7) == 12 {
                DiffKind::Added
            } else if index % (97 + file * 5) == 24 {
                DiffKind::Removed
            } else if index % (61 + file * 3) == 40 {
                DiffKind::Modified
            } else {
                DiffKind::Unchanged
            };
            let old = (kind != DiffKind::Added).then_some(old_number);
            let new = (kind != DiffKind::Removed).then_some(new_number);
            if old.is_some() {
                old_number += 1;
            }
            if new.is_some() {
                new_number += 1;
            }
            DiffLine {
                old_number: old,
                new_number: new,
                old_text: if kind == DiffKind::Added {
                    String::new()
                } else {
                    format!("let value_{index} = compute({});", index + file)
                },
                new_text: if kind == DiffKind::Removed {
                    String::new()
                } else if kind == DiffKind::Modified {
                    format!("let value_{index} = compute_checked({});", index + file)
                } else if kind == DiffKind::Added {
                    format!("let inserted_{index} = validate({});", index + file)
                } else {
                    format!("let value_{index} = compute({});", index + file)
                },
                kind,
            }
        })
        .collect()
}

fn diff_display_rows(lines: &[DiffLine], collapsed: bool) -> Vec<DiffDisplayRow> {
    if !collapsed {
        return (0..lines.len()).map(DiffDisplayRow::Line).collect();
    }
    let mut rows = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if lines[index].kind != DiffKind::Unchanged {
            rows.push(DiffDisplayRow::Line(index));
            index += 1;
            continue;
        }
        let start = index;
        while index < lines.len() && lines[index].kind == DiffKind::Unchanged {
            index += 1;
        }
        let count = index - start;
        if count <= 8 {
            rows.extend((start..index).map(DiffDisplayRow::Line));
        } else {
            rows.extend((start..start + 3).map(DiffDisplayRow::Line));
            rows.push(DiffDisplayRow::Fold {
                start: start + 3,
                count: count - 6,
            });
            rows.extend((index - 3..index).map(DiffDisplayRow::Line));
        }
    }
    rows
}

fn directional_focus(focus: Focus, key: &str) -> Focus {
    match (focus, key) {
        (Focus::List, "h") => Focus::Text,
        (Focus::Action | Focus::Toggle | Focus::Text, "l") => Focus::List,
        (Focus::Action, "j") => Focus::Toggle,
        (Focus::Toggle, "j") => Focus::Text,
        (Focus::Text, "k") => Focus::Toggle,
        (Focus::Toggle, "k") => Focus::Action,
        _ => focus,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FilterMode {
    Literal,
    Regex,
    Fuzzy,
}

impl FilterMode {
    fn next(self) -> Self {
        match self {
            Self::Fuzzy => Self::Regex,
            Self::Regex => Self::Literal,
            Self::Literal => Self::Fuzzy,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Literal => "Literal",
            Self::Regex => "Regex",
            Self::Fuzzy => "Fuzzy",
        }
    }
}

fn data_search_index() -> Vec<String> {
    (0..ITEM_COUNT)
        .map(|row| format!("{:04} canvas row {} {}", row, row + 1, row_status(row)))
        .collect()
}

fn fuzzy_score(candidate: &str, query: &str) -> Option<i64> {
    if query.is_empty() {
        return Some(0);
    }
    let mut score = 0;
    let mut cursor = 0;
    let mut previous = None;
    let candidate = candidate.to_ascii_lowercase();
    for wanted in query.to_ascii_lowercase().chars() {
        let found = candidate[cursor..].find(wanted)? + cursor;
        score += if previous == Some(found.saturating_sub(1)) {
            12
        } else {
            4
        };
        score -= found as i64;
        cursor = found + wanted.len_utf8();
        previous = Some(found);
    }
    Some(score)
}

fn filtered_data_rows(
    index: &[String],
    filter: &str,
    mode: FilterMode,
    descending: bool,
) -> Result<Vec<usize>, String> {
    let query = filter.trim();
    let mut rows: Vec<usize> = match mode {
        FilterMode::Literal => {
            let query = query.to_ascii_lowercase();
            index
                .iter()
                .enumerate()
                .filter_map(|(row, value)| {
                    (query.is_empty() || value.to_ascii_lowercase().contains(&query)).then_some(row)
                })
                .collect()
        }
        FilterMode::Regex => {
            let regex = RegexBuilder::new(query)
                .case_insensitive(true)
                .build()
                .map_err(|error| error.to_string())?;
            index
                .iter()
                .enumerate()
                .filter_map(|(row, value)| regex.is_match(value).then_some(row))
                .collect()
        }
        FilterMode::Fuzzy => index
            .iter()
            .enumerate()
            .filter_map(|(row, value)| fuzzy_score(value, query).map(|_| row))
            .collect(),
    };
    if descending {
        rows.reverse();
    }
    Ok(rows)
}

fn item_range(anchor: usize, active: usize) -> std::ops::RangeInclusive<usize> {
    anchor.min(active)..=anchor.max(active)
}

fn utf16_len(text: &str) -> u32 {
    text.encode_utf16().count() as u32
}

fn utf16_to_byte(text: &str, offset: u32) -> usize {
    let mut units = 0;
    for (byte, ch) in text.char_indices() {
        let next = units + ch.len_utf16() as u32;
        if offset < next {
            return byte;
        }
        units = next;
    }
    text.len()
}

fn line_for_utf16(text: &str, offset: u32) -> usize {
    let byte = utf16_to_byte(text, offset);
    text[..byte].bytes().filter(|byte| *byte == b'\n').count()
}

fn text_line_count(text: &str) -> usize {
    text.split('\n').count().max(1)
}

fn previous_grapheme_utf16(text: &str, offset: u32) -> u32 {
    let byte = utf16_to_byte(text, offset);
    text.grapheme_indices(true)
        .map(|(index, _)| utf16_len(&text[..index]))
        .take_while(|boundary| *boundary < utf16_len(&text[..byte]))
        .last()
        .unwrap_or(0)
}

fn next_grapheme_utf16(text: &str, offset: u32) -> u32 {
    let byte = utf16_to_byte(text, offset);
    let current = utf16_len(&text[..byte]);
    text.grapheme_indices(true)
        .map(|(index, _)| utf16_len(&text[..index]))
        .find(|boundary| *boundary > current)
        .unwrap_or_else(|| utf16_len(text))
}

fn find_utf16_matches(text: &str, query: &str) -> Vec<(u32, u32)> {
    if query.is_empty() {
        return Vec::new();
    }
    text.match_indices(query)
        .map(|(byte, value)| {
            let start = utf16_len(&text[..byte]);
            (start, start + utf16_len(value))
        })
        .collect()
}

fn line_bounds_for_utf16(text: &str, offset: u32) -> (u32, u32) {
    let mut start = 0;
    for line in text.split('\n') {
        let end = start + utf16_len(line);
        if offset <= end {
            return (start, end);
        }
        start = end + 1;
    }
    let end = utf16_len(text);
    (end, end)
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::*;
    use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
    use wasm_bindgen::{JsCast, closure::Closure, prelude::*};
    use web_sys::{
        AddEventListenerOptions, CanvasRenderingContext2d, Event, HtmlCanvasElement, HtmlElement,
        HtmlTextAreaElement, InputEvent, KeyboardEvent, PointerEvent, WheelEvent,
    };

    const EDITOR_FONT: &str = "15px ui-monospace, SFMono-Regular, Consolas, monospace";
    const EDITOR_LINE_HEIGHT: f64 = 24.0;
    const EDITOR_PADDING: f64 = 14.0;
    const COMMANDS: [&str; 3] = [
        "Select all document text",
        "Clear search",
        "Toggle custom rendering",
    ];
    const DIFF_FILES: [&str; 4] = [
        "src/editor.rs",
        "src/renderer.rs",
        "src/platform/web.rs",
        "tests/workbench.rs",
    ];
    const SPEAKERS: [&str; 4] = ["Ada", "Ben", "Chen", "Dia"];
    const SHORTCUTS: [(&str, &str); 21] = [
        ("Ctrl/Cmd+F", "Search document or filter focused data"),
        ("Ctrl/Cmd+P", "Open command palette"),
        (
            "Ctrl/Cmd+1 / 3 / 4",
            "Open Document+Data / Diff / Transcript",
        ),
        ("?", "Show keyboard shortcuts"),
        ("Escape", "Close the active overlay"),
        ("Arrow keys", "Move in the document or list"),
        ("j / k", "Select next / previous list item"),
        ("h / l", "Select first / last list item"),
        ("Shift+movement", "Extend the selection"),
        ("Ctrl/Cmd+Click", "Toggle a list item"),
        ("Alt+H/J/K/L", "Move focus between UI panes"),
        ("/", "Filter data rows"),
        ("m", "Cycle Literal / Regex / Fuzzy mode"),
        ("s", "Toggle data sort order"),
        ("r", "Mark selected rows reviewed"),
        ("n / p", "Next / previous diff change"),
        ("c", "Collapse or expand unchanged diff lines"),
        ("/", "Fuzzy search transcript segments"),
        ("Enter", "Edit selected transcript segment"),
        ("Space", "Play or pause transcript"),
        ("f", "Toggle follow playback"),
    ];

    fn matching_commands(query: &str) -> Vec<usize> {
        let query = query.to_ascii_lowercase();
        COMMANDS
            .iter()
            .enumerate()
            .filter_map(|(index, command)| {
                command
                    .to_ascii_lowercase()
                    .contains(&query)
                    .then_some(index)
            })
            .collect()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Overlay {
        None,
        Search,
        Command,
        Shortcuts,
        DataFilter,
        TranscriptSearch,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Hover {
        Action,
        Toggle,
        Text,
        DataFilter,
        DataFilterClear,
        DataFilterMode,
        DataSort,
        DataBatch,
        Row(usize),
        ModeDocumentData,
        ModeDiff,
        ModeTranscript,
        DiffFile(usize),
        DiffRow(usize),
        DiffPrevious,
        DiffNext,
        DiffCollapse,
        TranscriptSpeaker(usize),
        TranscriptSegment(usize),
        TranscriptSearch,
        TranscriptPlay,
        TranscriptFollow,
    }

    #[derive(Clone, Copy)]
    struct Layout {
        action: Rect,
        toggle: Rect,
        text: Rect,
        list: Rect,
        data_filter: Rect,
        data_filter_mode: Rect,
        data_sort: Rect,
        data_batch: Rect,
        list_content: Rect,
    }

    fn mode_document_data_rect() -> Rect {
        Rect {
            x: 24.0,
            y: 64.0,
            width: 132.0,
            height: 24.0,
        }
    }

    fn mode_diff_rect() -> Rect {
        Rect {
            x: 164.0,
            y: 64.0,
            width: 72.0,
            height: 24.0,
        }
    }

    fn mode_transcript_rect() -> Rect {
        Rect {
            x: 244.0,
            y: 64.0,
            width: 104.0,
            height: 24.0,
        }
    }

    fn data_filter_clear_rect(filter: Rect) -> Rect {
        Rect {
            x: filter.x + filter.width - 28.0,
            y: filter.y,
            width: 28.0,
            height: filter.height,
        }
    }

    fn data_controls(list: Rect) -> (Rect, Rect, Rect, Rect) {
        let gap = 8.0;
        let available = list.width - 48.0;
        let filter_width = available * 0.38;
        let mode_width = available * 0.16;
        let sort_width = available * 0.14;
        let batch_width = available - filter_width - mode_width - sort_width - gap * 3.0;
        let x = list.x + 12.0;
        let y = list.y + 48.0;
        let filter = Rect {
            x,
            y,
            width: filter_width,
            height: 28.0,
        };
        let mode = Rect {
            x: filter.x + filter.width + gap,
            y,
            width: mode_width,
            height: 28.0,
        };
        let sort = Rect {
            x: mode.x + mode.width + gap,
            y,
            width: sort_width,
            height: 28.0,
        };
        let batch = Rect {
            x: sort.x + sort.width + gap,
            y,
            width: batch_width,
            height: 28.0,
        };
        (filter, mode, sort, batch)
    }

    #[derive(Clone, Copy)]
    struct DiffLayout {
        files: Rect,
        files_content: Rect,
        content: Rect,
        content_body: Rect,
        previous: Rect,
        next: Rect,
        collapse: Rect,
    }

    fn diff_rows_rect(layout: DiffLayout) -> Rect {
        Rect {
            x: layout.content_body.x,
            y: layout.content_body.y + 26.0,
            width: layout.content_body.width,
            height: (layout.content_body.height - 26.0).max(0.0),
        }
    }

    #[derive(Clone, Copy)]
    struct TranscriptLayout {
        speakers: Rect,
        speakers_content: Rect,
        content: Rect,
        content_body: Rect,
        search: Rect,
        play: Rect,
        follow: Rect,
    }

    fn transcript_rows_rect(layout: TranscriptLayout) -> Rect {
        Rect {
            x: layout.content_body.x,
            y: layout.content_body.y + 26.0,
            width: layout.content_body.width,
            height: (layout.content_body.height - 26.0).max(0.0),
        }
    }

    struct Lab {
        canvas: HtmlCanvasElement,
        input: HtmlTextAreaElement,
        a11y_mode_document: HtmlElement,
        a11y_mode_diff: HtmlElement,
        a11y_mode_transcript: HtmlElement,
        a11y_transcript_search: HtmlElement,
        a11y_transcript_play: HtmlElement,
        a11y_transcript_follow: HtmlElement,
        a11y_diff_previous: HtmlElement,
        a11y_diff_next: HtmlElement,
        a11y_diff_collapse: HtmlElement,
        a11y_action: HtmlElement,
        a11y_toggle: HtmlElement,
        a11y_list: HtmlElement,
        a11y_option: HtmlElement,
        a11y_status: HtmlElement,
        context: CanvasRenderingContext2d,
        mode: WorkbenchMode,
        width: f64,
        height: f64,
        dpr: f64,
        pointer_x: f64,
        pointer_y: f64,
        scroll: f64,
        selected: usize,
        selected_items: BTreeSet<usize>,
        list_anchor: usize,
        action_count: u32,
        enabled: bool,
        text: String,
        composing: bool,
        composition_anchor: Option<u32>,
        selection_start: u32,
        selection_end: u32,
        selection_backward: bool,
        drag_anchor: Option<u32>,
        preferred_editor_x: Option<f64>,
        delegated_native_navigation: bool,
        editor_scroll: f64,
        overlay: Overlay,
        search_query: String,
        active_match: usize,
        command_query: String,
        command_selected: usize,
        data_filter: String,
        data_filter_mode: FilterMode,
        data_filter_error: Option<String>,
        data_filter_ms: f64,
        data_filter_generation: u32,
        data_filter_pending: bool,
        data_filter_modal: bool,
        data_descending: bool,
        data_index: Vec<String>,
        data_rows: Vec<usize>,
        reviewed_items: BTreeSet<usize>,
        diff_file: usize,
        diff_lines: Vec<DiffLine>,
        diff_scroll: f64,
        diff_selected: usize,
        diff_anchor: usize,
        diff_selection_start: usize,
        diff_selection_end: usize,
        diff_collapsed: bool,
        diff_pane: DiffPane,
        transcript_segments: Vec<TranscriptSegment>,
        transcript_scroll: f64,
        transcript_selected: usize,
        transcript_anchor: usize,
        transcript_selected_items: BTreeSet<usize>,
        transcript_speaker_filter: Option<usize>,
        transcript_pane: TranscriptPane,
        transcript_query: String,
        transcript_active_match: usize,
        transcript_playback: usize,
        transcript_playing: bool,
        transcript_follow: bool,
        transcript_edit_target: Option<usize>,
        last_input: String,
        focus: Focus,
        hover: Option<Hover>,
        render_pending: bool,
        last_render_ms: f64,
    }

    impl Lab {
        fn layout(&self) -> Layout {
            let margin = 24.0;
            let top = 88.0;
            if self.width >= 760.0 {
                let controls_width = 286.0;
                let list = Rect {
                    x: margin + controls_width + 20.0,
                    y: top,
                    width: (self.width - controls_width - margin * 2.0 - 20.0).max(240.0),
                    height: (self.height - top - margin).max(220.0),
                };
                let (data_filter, data_filter_mode, data_sort, data_batch) = data_controls(list);
                Layout {
                    action: Rect {
                        x: margin + 18.0,
                        y: top + 68.0,
                        width: controls_width - 36.0,
                        height: 44.0,
                    },
                    toggle: Rect {
                        x: margin + 18.0,
                        y: top + 132.0,
                        width: controls_width - 36.0,
                        height: 42.0,
                    },
                    text: Rect {
                        x: margin + 18.0,
                        y: top + 214.0,
                        width: controls_width - 36.0,
                        height: 190.0,
                    },
                    list,
                    data_filter,
                    data_filter_mode,
                    data_sort,
                    data_batch,
                    list_content: Rect {
                        x: list.x + 1.0,
                        y: list.y + 88.0,
                        width: list.width - 2.0,
                        height: list.height - 89.0,
                    },
                }
            } else {
                let controls_width = self.width - margin * 2.0;
                let list = Rect {
                    x: margin,
                    y: top + 410.0,
                    width: controls_width,
                    height: (self.height - top - 434.0).max(170.0),
                };
                let (data_filter, data_filter_mode, data_sort, data_batch) = data_controls(list);
                Layout {
                    action: Rect {
                        x: margin + 16.0,
                        y: top + 48.0,
                        width: controls_width - 32.0,
                        height: 40.0,
                    },
                    toggle: Rect {
                        x: margin + 16.0,
                        y: top + 102.0,
                        width: controls_width - 32.0,
                        height: 38.0,
                    },
                    text: Rect {
                        x: margin + 16.0,
                        y: top + 186.0,
                        width: controls_width - 32.0,
                        height: 150.0,
                    },
                    list,
                    data_filter,
                    data_filter_mode,
                    data_sort,
                    data_batch,
                    list_content: Rect {
                        x: list.x + 1.0,
                        y: list.y + 88.0,
                        width: list.width - 2.0,
                        height: list.height - 89.0,
                    },
                }
            }
        }

        fn diff_layout(&self) -> DiffLayout {
            let margin = 24.0;
            let top = 96.0;
            let files_width = 240.0;
            let files = Rect {
                x: margin,
                y: top,
                width: files_width,
                height: (self.height - top - margin).max(220.0),
            };
            let content = Rect {
                x: files.x + files.width + 16.0,
                y: top,
                width: (self.width - files.width - margin * 2.0 - 16.0).max(360.0),
                height: files.height,
            };
            let button_width = 84.0;
            DiffLayout {
                files,
                files_content: Rect {
                    x: files.x + 1.0,
                    y: files.y + 46.0,
                    width: files.width - 2.0,
                    height: files.height - 47.0,
                },
                content,
                content_body: Rect {
                    x: content.x + 1.0,
                    y: content.y + 58.0,
                    width: content.width - 2.0,
                    height: content.height - 59.0,
                },
                previous: Rect {
                    x: content.x + content.width - button_width * 3.0 - 32.0,
                    y: content.y + 14.0,
                    width: button_width,
                    height: 30.0,
                },
                next: Rect {
                    x: content.x + content.width - button_width * 2.0 - 24.0,
                    y: content.y + 14.0,
                    width: button_width,
                    height: 30.0,
                },
                collapse: Rect {
                    x: content.x + content.width - button_width - 16.0,
                    y: content.y + 14.0,
                    width: button_width,
                    height: 30.0,
                },
            }
        }

        fn transcript_layout(&self) -> TranscriptLayout {
            let margin = 24.0;
            let top = 96.0;
            let speakers = Rect {
                x: margin,
                y: top,
                width: 220.0,
                height: (self.height - top - margin).max(220.0),
            };
            let content = Rect {
                x: speakers.x + speakers.width + 16.0,
                y: top,
                width: (self.width - speakers.width - margin * 2.0 - 16.0).max(380.0),
                height: speakers.height,
            };
            TranscriptLayout {
                speakers,
                speakers_content: Rect {
                    x: speakers.x + 1.0,
                    y: speakers.y + 46.0,
                    width: speakers.width - 2.0,
                    height: speakers.height - 47.0,
                },
                content,
                content_body: Rect {
                    x: content.x + 1.0,
                    y: content.y + 58.0,
                    width: content.width - 2.0,
                    height: content.height - 59.0,
                },
                search: Rect {
                    x: content.x + content.width - 302.0,
                    y: content.y + 14.0,
                    width: 110.0,
                    height: 30.0,
                },
                play: Rect {
                    x: content.x + content.width - 184.0,
                    y: content.y + 14.0,
                    width: 80.0,
                    height: 30.0,
                },
                follow: Rect {
                    x: content.x + content.width - 96.0,
                    y: content.y + 14.0,
                    width: 80.0,
                    height: 30.0,
                },
            }
        }

        fn transcript_rows(&self) -> Vec<usize> {
            (0..self.transcript_segments.len())
                .filter(|index| {
                    self.transcript_speaker_filter
                        .is_none_or(|speaker| self.transcript_segments[*index].speaker == speaker)
                })
                .collect()
        }

        fn hover_at(&self, x: f64, y: f64) -> Option<Hover> {
            if mode_document_data_rect().contains(x, y) {
                return Some(Hover::ModeDocumentData);
            }
            if mode_diff_rect().contains(x, y) {
                return Some(Hover::ModeDiff);
            }
            if mode_transcript_rect().contains(x, y) {
                return Some(Hover::ModeTranscript);
            }
            if self.mode == WorkbenchMode::Transcript {
                let layout = self.transcript_layout();
                if layout.search.contains(x, y) {
                    return Some(Hover::TranscriptSearch);
                }
                if layout.play.contains(x, y) {
                    return Some(Hover::TranscriptPlay);
                }
                if layout.follow.contains(x, y) {
                    return Some(Hover::TranscriptFollow);
                }
                if layout.speakers_content.contains(x, y) {
                    let speaker = ((y - layout.speakers_content.y) / 40.0) as usize;
                    return (speaker <= SPEAKERS.len())
                        .then_some(Hover::TranscriptSpeaker(speaker));
                }
                let rows_rect = transcript_rows_rect(layout);
                if rows_rect.contains(x, y) {
                    let position =
                        ((y - rows_rect.y + self.transcript_scroll) / ROW_HEIGHT) as usize;
                    let rows = self.transcript_rows();
                    return rows.get(position).copied().map(Hover::TranscriptSegment);
                }
                return None;
            }
            if self.mode == WorkbenchMode::Diff {
                let layout = self.diff_layout();
                if layout.previous.contains(x, y) {
                    return Some(Hover::DiffPrevious);
                }
                if layout.next.contains(x, y) {
                    return Some(Hover::DiffNext);
                }
                if layout.collapse.contains(x, y) {
                    return Some(Hover::DiffCollapse);
                }
                if layout.files_content.contains(x, y) {
                    let file = ((y - layout.files_content.y) / 38.0) as usize;
                    return (file < DIFF_FILES.len()).then_some(Hover::DiffFile(file));
                }
                let rows_rect = diff_rows_rect(layout);
                if rows_rect.contains(x, y) {
                    let row = ((y - rows_rect.y + self.diff_scroll) / ROW_HEIGHT) as usize;
                    let count = diff_display_rows(&self.diff_lines, self.diff_collapsed).len();
                    return (row < count).then_some(Hover::DiffRow(row));
                }
                return None;
            }
            let layout = self.layout();
            if layout.action.contains(x, y) {
                Some(Hover::Action)
            } else if layout.toggle.contains(x, y) {
                Some(Hover::Toggle)
            } else if layout.text.contains(x, y) {
                Some(Hover::Text)
            } else if data_filter_clear_rect(layout.data_filter).contains(x, y) {
                Some(Hover::DataFilterClear)
            } else if layout.data_filter.contains(x, y) {
                Some(Hover::DataFilter)
            } else if layout.data_filter_mode.contains(x, y) {
                Some(Hover::DataFilterMode)
            } else if layout.data_sort.contains(x, y) {
                Some(Hover::DataSort)
            } else if layout.data_batch.contains(x, y) {
                Some(Hover::DataBatch)
            } else if layout.list_content.contains(x, y) {
                let position = ((y - layout.list_content.y + self.scroll) / ROW_HEIGHT) as usize;
                self.data_rows.get(position).copied().map(Hover::Row)
            } else {
                None
            }
        }

        fn select_list_row(&mut self, row: usize, extend: bool, toggle: bool) {
            self.selected = row;
            if extend {
                let anchor = self
                    .data_rows
                    .iter()
                    .position(|candidate| *candidate == self.list_anchor);
                let active = self
                    .data_rows
                    .iter()
                    .position(|candidate| *candidate == row);
                self.selected_items.clear();
                if let (Some(anchor), Some(active)) = (anchor, active) {
                    self.selected_items
                        .extend(self.data_rows[item_range(anchor, active)].iter().copied());
                } else {
                    self.selected_items.insert(row);
                    self.list_anchor = row;
                }
            } else if toggle {
                if !self.selected_items.remove(&row) {
                    self.selected_items.insert(row);
                }
                self.list_anchor = row;
            } else {
                self.selected_items.clear();
                self.selected_items.insert(row);
                self.list_anchor = row;
            }
        }

        fn select_diff_row(&mut self, row: usize, extend: bool) {
            let count = diff_display_rows(&self.diff_lines, self.diff_collapsed).len();
            let row = row.min(count.saturating_sub(1));
            self.diff_selected = row;
            if extend {
                self.diff_selection_start = self.diff_anchor.min(row);
                self.diff_selection_end = self.diff_anchor.max(row);
            } else {
                self.diff_anchor = row;
                self.diff_selection_start = row;
                self.diff_selection_end = row;
            }
            self.ensure_diff_visible();
        }

        fn ensure_diff_visible(&mut self) {
            let viewport = diff_rows_rect(self.diff_layout()).height;
            let top = self.diff_selected as f64 * ROW_HEIGHT;
            let bottom = top + ROW_HEIGHT;
            if top < self.diff_scroll {
                self.diff_scroll = top;
            } else if bottom > self.diff_scroll + viewport {
                self.diff_scroll = bottom - viewport;
            }
            let count = diff_display_rows(&self.diff_lines, self.diff_collapsed).len();
            self.diff_scroll = clamp_scroll_for(count, self.diff_scroll, viewport);
        }

        fn select_diff_change(&mut self, forward: bool) {
            let rows = diff_display_rows(&self.diff_lines, self.diff_collapsed);
            let is_change = |row: &DiffDisplayRow| match row {
                DiffDisplayRow::Line(index) => self.diff_lines[*index].kind != DiffKind::Unchanged,
                DiffDisplayRow::Fold { .. } => false,
            };
            let target = if forward {
                rows.iter()
                    .enumerate()
                    .skip(self.diff_selected + 1)
                    .find(|(_, row)| is_change(row))
                    .map(|(index, _)| index)
                    .or_else(|| {
                        rows.iter()
                            .enumerate()
                            .find(|(_, row)| is_change(row))
                            .map(|(index, _)| index)
                    })
            } else {
                rows.iter()
                    .enumerate()
                    .take(self.diff_selected)
                    .rev()
                    .find(|(_, row)| is_change(row))
                    .map(|(index, _)| index)
                    .or_else(|| {
                        rows.iter()
                            .enumerate()
                            .rev()
                            .find(|(_, row)| is_change(row))
                            .map(|(index, _)| index)
                    })
            };
            if let Some(target) = target {
                self.select_diff_row(target, false);
            }
        }

        fn toggle_diff_collapse(&mut self) {
            let old_rows = diff_display_rows(&self.diff_lines, self.diff_collapsed);
            let selected_line = old_rows.get(self.diff_selected).map(|row| match row {
                DiffDisplayRow::Line(index) => *index,
                DiffDisplayRow::Fold { start, .. } => *start,
            });
            self.diff_collapsed = !self.diff_collapsed;
            let rows = diff_display_rows(&self.diff_lines, self.diff_collapsed);
            let target = selected_line
                .and_then(|line| {
                    rows.iter().position(|row| match row {
                        DiffDisplayRow::Line(index) => *index == line,
                        DiffDisplayRow::Fold { start, count } => {
                            line >= *start && line < *start + *count
                        }
                    })
                })
                .unwrap_or(0);
            self.select_diff_row(target, false);
        }

        fn set_diff_file(&mut self, file: usize) {
            self.diff_file = file.min(DIFF_FILES.len() - 1);
            self.diff_lines = generate_diff(self.diff_file, 5_000);
            self.diff_scroll = 0.0;
            self.select_diff_row(0, false);
        }

        fn select_transcript_segment(&mut self, segment: usize, extend: bool, toggle: bool) {
            self.transcript_selected = segment.min(self.transcript_segments.len() - 1);
            if extend {
                let rows = self.transcript_rows();
                let anchor = rows
                    .iter()
                    .position(|index| *index == self.transcript_anchor);
                let active = rows
                    .iter()
                    .position(|index| *index == self.transcript_selected);
                self.transcript_selected_items.clear();
                if let (Some(anchor), Some(active)) = (anchor, active) {
                    self.transcript_selected_items
                        .extend(rows[item_range(anchor, active)].iter().copied());
                } else {
                    self.transcript_selected_items
                        .insert(self.transcript_selected);
                }
            } else if toggle {
                if !self
                    .transcript_selected_items
                    .remove(&self.transcript_selected)
                {
                    self.transcript_selected_items
                        .insert(self.transcript_selected);
                }
                self.transcript_anchor = self.transcript_selected;
            } else {
                self.transcript_selected_items.clear();
                self.transcript_selected_items
                    .insert(self.transcript_selected);
                self.transcript_anchor = self.transcript_selected;
            }
            self.ensure_transcript_visible();
        }

        fn ensure_transcript_visible(&mut self) {
            let rows = self.transcript_rows();
            let Some(position) = rows
                .iter()
                .position(|index| *index == self.transcript_selected)
            else {
                self.transcript_scroll = 0.0;
                return;
            };
            let viewport = transcript_rows_rect(self.transcript_layout()).height;
            let top = position as f64 * ROW_HEIGHT;
            let bottom = top + ROW_HEIGHT;
            if top < self.transcript_scroll {
                self.transcript_scroll = top;
            } else if bottom > self.transcript_scroll + viewport {
                self.transcript_scroll = bottom - viewport;
            }
            self.transcript_scroll = clamp_scroll_for(rows.len(), self.transcript_scroll, viewport);
        }

        fn select_transcript_match(&mut self, delta: isize) {
            let matches = transcript_matches(&self.transcript_segments, &self.transcript_query);
            if matches.is_empty() {
                self.transcript_active_match = 0;
                return;
            }
            self.transcript_active_match = (self.transcript_active_match as isize + delta)
                .rem_euclid(matches.len() as isize)
                as usize;
            self.transcript_speaker_filter = None;
            self.select_transcript_segment(matches[self.transcript_active_match], false, false);
            self.transcript_pane = TranscriptPane::Segments;
        }

        fn advance_transcript_playback(&mut self) {
            if !self.transcript_playing || self.transcript_segments.is_empty() {
                return;
            }
            self.transcript_playback =
                (self.transcript_playback + 1) % self.transcript_segments.len();
            if self.transcript_follow {
                self.transcript_speaker_filter = None;
                self.select_transcript_segment(self.transcript_playback, false, false);
                self.transcript_pane = TranscriptPane::Segments;
            }
        }

        fn sync_accessibility(&self) {
            let _ = self.a11y_mode_document.set_attribute(
                "aria-pressed",
                if self.mode == WorkbenchMode::DocumentData {
                    "true"
                } else {
                    "false"
                },
            );
            let _ = self.a11y_mode_diff.set_attribute(
                "aria-pressed",
                if self.mode == WorkbenchMode::Diff {
                    "true"
                } else {
                    "false"
                },
            );
            let _ = self.a11y_mode_transcript.set_attribute(
                "aria-pressed",
                if self.mode == WorkbenchMode::Transcript {
                    "true"
                } else {
                    "false"
                },
            );
            for control in [
                &self.a11y_diff_previous,
                &self.a11y_diff_next,
                &self.a11y_diff_collapse,
            ] {
                if self.mode == WorkbenchMode::Diff {
                    let _ = control.remove_attribute("hidden");
                } else {
                    let _ = control.set_attribute("hidden", "");
                }
            }
            for control in [
                &self.a11y_transcript_search,
                &self.a11y_transcript_play,
                &self.a11y_transcript_follow,
            ] {
                if self.mode == WorkbenchMode::Transcript {
                    let _ = control.remove_attribute("hidden");
                } else {
                    let _ = control.set_attribute("hidden", "");
                }
            }
            self.a11y_transcript_play
                .set_text_content(Some(if self.transcript_playing {
                    "Pause transcript playback"
                } else {
                    "Play transcript"
                }));
            let _ = self.a11y_transcript_play.set_attribute(
                "aria-pressed",
                if self.transcript_playing {
                    "true"
                } else {
                    "false"
                },
            );
            let _ = self.a11y_transcript_follow.set_attribute(
                "aria-pressed",
                if self.transcript_follow {
                    "true"
                } else {
                    "false"
                },
            );

            let _ = self.a11y_list.set_attribute(
                "aria-multiselectable",
                if self.mode == WorkbenchMode::Transcript
                    && self.transcript_pane == TranscriptPane::Speakers
                {
                    "false"
                } else {
                    "true"
                },
            );

            let _ = self.a11y_diff_collapse.set_attribute(
                "aria-expanded",
                if self.diff_collapsed { "false" } else { "true" },
            );
            self.a11y_diff_collapse
                .set_text_content(Some(if self.diff_collapsed {
                    "Expand unchanged diff lines"
                } else {
                    "Collapse unchanged diff lines"
                }));

            if self.mode == WorkbenchMode::Transcript {
                let _ = self.a11y_action.set_attribute("hidden", "");
                let _ = self.a11y_toggle.set_attribute("hidden", "");
                if self.overlay == Overlay::TranscriptSearch
                    || self.transcript_edit_target.is_some()
                {
                    let _ = self.input.remove_attribute("hidden");
                } else {
                    let _ = self.input.set_attribute("hidden", "");
                }
                let input_label = if self.overlay == Overlay::TranscriptSearch {
                    "Fuzzy search transcript"
                } else if self.transcript_edit_target.is_some() {
                    "Edit transcript segment"
                } else {
                    "Selected transcript segment"
                };
                let _ = self.input.set_attribute("aria-label", input_label);
                let rows = self.transcript_rows();
                let list_label = if self.transcript_pane == TranscriptPane::Speakers {
                    format!("Transcript speakers, {} options", SPEAKERS.len() + 1)
                } else {
                    format!("Transcript segments, {} visible", rows.len())
                };
                let _ = self.a11y_list.set_attribute("aria-label", &list_label);
                let speaker_pane = self.transcript_pane == TranscriptPane::Speakers;
                self.a11y_option.set_id("active-transcript-item");
                let _ = self.a11y_option.remove_attribute("aria-hidden");
                let _ = self
                    .a11y_list
                    .set_attribute("aria-activedescendant", "active-transcript-item");
                let (position, set_size, description) =
                    if self.transcript_pane == TranscriptPane::Speakers {
                        let position = self.transcript_speaker_filter.map_or(0, |value| value + 1);
                        (
                            position + 1,
                            SPEAKERS.len() + 1,
                            if position == 0 {
                                "All speakers".into()
                            } else {
                                format!("Speaker {}", SPEAKERS[position - 1])
                            },
                        )
                    } else {
                        let position = rows
                            .iter()
                            .position(|index| *index == self.transcript_selected)
                            .unwrap_or(0);
                        let segment = &self.transcript_segments[self.transcript_selected];
                        (
                            position + 1,
                            rows.len(),
                            format!(
                                "Segment {} of {}, {}, speaker {}. {}",
                                position + 1,
                                rows.len(),
                                format_timestamp(segment.seconds),
                                SPEAKERS[segment.speaker],
                                segment.text
                            ),
                        )
                    };
                self.a11y_option.set_text_content(Some(&description));
                let _ = self
                    .a11y_option
                    .set_attribute("aria-posinset", &position.to_string());
                let _ = self
                    .a11y_option
                    .set_attribute("aria-setsize", &set_size.to_string());
                let option_selected = speaker_pane
                    || self
                        .transcript_selected_items
                        .contains(&self.transcript_selected);
                let _ = self.a11y_option.set_attribute(
                    "aria-selected",
                    if option_selected { "true" } else { "false" },
                );
                let status = if self.overlay == Overlay::TranscriptSearch {
                    let count =
                        transcript_matches(&self.transcript_segments, &self.transcript_query).len();
                    if count == 0 {
                        "No transcript matches".into()
                    } else {
                        format!(
                            "Transcript match {} of {}",
                            self.transcript_active_match.min(count - 1) + 1,
                            count
                        )
                    }
                } else {
                    format!(
                        "{} segments selected. Playback {} at {}. Follow {}",
                        self.transcript_selected_items.len(),
                        if self.transcript_playing {
                            "playing"
                        } else {
                            "paused"
                        },
                        format_timestamp(
                            self.transcript_segments[self.transcript_playback].seconds
                        ),
                        if self.transcript_follow { "on" } else { "off" }
                    )
                };
                self.a11y_status.set_text_content(Some(&status));
                return;
            }

            if self.mode == WorkbenchMode::Diff {
                let _ = self.a11y_action.set_attribute("hidden", "");
                let _ = self.a11y_toggle.set_attribute("hidden", "");
                let _ = self.input.set_attribute("hidden", "");
                let rows = diff_display_rows(&self.diff_lines, self.diff_collapsed);
                let _ = self.a11y_list.set_attribute(
                    "aria-label",
                    &format!(
                        "Diff viewer, {}, {} pane, {} displayed rows",
                        DIFF_FILES[self.diff_file],
                        if self.diff_pane == DiffPane::Files {
                            "files"
                        } else {
                            "content"
                        },
                        rows.len()
                    ),
                );
                self.a11y_option.set_id("active-diff-item");
                let _ = self.a11y_option.remove_attribute("aria-hidden");
                let _ = self
                    .a11y_list
                    .set_attribute("aria-activedescendant", "active-diff-item");
                let (position, set_size, description) = if self.diff_pane == DiffPane::Files {
                    (
                        self.diff_file + 1,
                        DIFF_FILES.len(),
                        format!(
                            "File {} of {}: {}",
                            self.diff_file + 1,
                            DIFF_FILES.len(),
                            DIFF_FILES[self.diff_file]
                        ),
                    )
                } else {
                    (
                        self.diff_selected + 1,
                        rows.len(),
                        match rows.get(self.diff_selected) {
                            Some(DiffDisplayRow::Line(index)) => {
                                let line = &self.diff_lines[*index];
                                let content = match line.kind {
                                    DiffKind::Added => format!("New: {}", line.new_text),
                                    DiffKind::Removed => format!("Old: {}", line.old_text),
                                    DiffKind::Modified => {
                                        format!("Old: {}. New: {}", line.old_text, line.new_text)
                                    }
                                    DiffKind::Unchanged => {
                                        format!("Unchanged: {}", line.new_text)
                                    }
                                };
                                format!(
                                    "Diff row {} of {}, {:?}, old line {}, new line {}. {}",
                                    self.diff_selected + 1,
                                    rows.len(),
                                    line.kind,
                                    line.old_number
                                        .map_or("none".into(), |value| value.to_string()),
                                    line.new_number
                                        .map_or("none".into(), |value| value.to_string()),
                                    content
                                )
                            }
                            Some(DiffDisplayRow::Fold { count, .. }) => {
                                format!("{} unchanged lines collapsed", count)
                            }
                            None => "No diff rows".into(),
                        },
                    )
                };
                self.a11y_option.set_text_content(Some(&description));
                let _ = self
                    .a11y_option
                    .set_attribute("aria-posinset", &position.to_string());
                let _ = self
                    .a11y_option
                    .set_attribute("aria-setsize", &set_size.to_string());
                let _ = self.a11y_option.set_attribute("aria-selected", "true");
                let changes = self
                    .diff_lines
                    .iter()
                    .filter(|line| line.kind != DiffKind::Unchanged)
                    .count();
                let selection_count = self.diff_selection_end - self.diff_selection_start + 1;
                self.a11y_status.set_text_content(Some(&format!(
                    "{} changes. {} diff row{} selected. Unchanged sections {}",
                    changes,
                    selection_count,
                    if selection_count == 1 { "" } else { "s" },
                    if self.diff_collapsed {
                        "collapsed"
                    } else {
                        "expanded"
                    }
                )));
                return;
            }
            let _ = self.a11y_action.remove_attribute("hidden");
            let _ = self.a11y_toggle.remove_attribute("hidden");
            let _ = self.input.remove_attribute("hidden");
            let _ = self.a11y_action.set_attribute(
                "aria-label",
                &format!("Record action, {} recorded", self.action_count),
            );
            let _ = self
                .a11y_toggle
                .set_attribute("aria-checked", if self.enabled { "true" } else { "false" });
            let rows = &self.data_rows;
            let _ = self.a11y_list.set_attribute(
                "aria-label",
                &format!(
                    "Data list, {} rows, {} filter mode, sorted {}{}",
                    rows.len(),
                    self.data_filter_mode.label(),
                    if self.data_descending {
                        "descending"
                    } else {
                        "ascending"
                    },
                    if self.data_filter.is_empty() {
                        String::new()
                    } else {
                        format!(", filtered by {}", self.data_filter)
                    }
                ),
            );
            if rows.is_empty() {
                let _ = self.a11y_list.remove_attribute("aria-activedescendant");
                let _ = self.a11y_option.set_attribute("aria-hidden", "true");
                self.a11y_option.set_text_content(Some("No matching rows"));
            } else {
                let selected = self.selected_items.contains(&self.selected);
                let option_id = format!("list-option-{}", self.selected);
                self.a11y_option.set_id(&option_id);
                let _ = self.a11y_option.remove_attribute("aria-hidden");
                let _ = self
                    .a11y_list
                    .set_attribute("aria-activedescendant", &option_id);
                let position = rows
                    .iter()
                    .position(|row| *row == self.selected)
                    .unwrap_or(0);
                self.a11y_option.set_text_content(Some(&format!(
                    "Canvas row {}, {}, {}",
                    self.selected + 1,
                    row_status(self.selected),
                    if self.reviewed_items.contains(&self.selected) {
                        "reviewed"
                    } else {
                        "not reviewed"
                    }
                )));
                let _ = self
                    .a11y_option
                    .set_attribute("aria-posinset", &(position + 1).to_string());
                let _ = self
                    .a11y_option
                    .set_attribute("aria-setsize", &rows.len().to_string());
                let _ = self
                    .a11y_option
                    .set_attribute("aria-selected", if selected { "true" } else { "false" });
            }
            let status = match self.overlay {
                Overlay::Search => {
                    let count = find_utf16_matches(&self.text, &self.search_query).len();
                    if count == 0 {
                        "No search matches".into()
                    } else {
                        format!(
                            "Match {} of {}",
                            self.active_match.min(count - 1) + 1,
                            count
                        )
                    }
                }
                Overlay::Command => {
                    let commands = matching_commands(&self.command_query);
                    commands
                        .get(self.command_selected.min(commands.len().saturating_sub(1)))
                        .map(|index| format!("Command: {}", COMMANDS[*index]))
                        .unwrap_or_else(|| "No matching commands".into())
                }
                Overlay::DataFilter if self.data_filter_pending => {
                    "Waiting to apply regular expression".into()
                }
                Overlay::DataFilter => self.data_filter_error.as_ref().map_or_else(
                    || {
                        format!(
                            "{} data rows match the {} filter in {:.2} milliseconds",
                            rows.len(),
                            self.data_filter_mode.label(),
                            self.data_filter_ms
                        )
                    },
                    |error| format!("Invalid regular expression: {error}"),
                ),
                Overlay::TranscriptSearch => "Transcript search".into(),
                Overlay::Shortcuts => format!(
                    "Keyboard shortcuts. {}",
                    SHORTCUTS
                        .iter()
                        .map(|(key, action)| format!("{key}: {action}"))
                        .collect::<Vec<_>>()
                        .join(". ")
                ),
                Overlay::None => format!("{} items selected", self.selected_items.len()),
            };
            if self.a11y_status.text_content().as_deref() != Some(&status) {
                self.a11y_status.set_text_content(Some(&status));
            }
        }

        fn ensure_selected_visible(&mut self) {
            let viewport = self.layout().list_content.height;
            let Some(position) = self.data_rows.iter().position(|row| *row == self.selected) else {
                self.scroll = 0.0;
                return;
            };
            let top = position as f64 * ROW_HEIGHT;
            let bottom = top + ROW_HEIGHT;
            if top < self.scroll {
                self.scroll = top;
            } else if bottom > self.scroll + viewport {
                self.scroll = bottom - viewport;
            }
            self.scroll = clamp_scroll_for(self.data_rows.len(), self.scroll, viewport);
        }

        fn refresh_data_rows(&mut self) {
            self.data_filter_generation = self.data_filter_generation.wrapping_add(1);
            self.data_filter_pending = false;
            let started = web_sys::window()
                .and_then(|window| window.performance())
                .map(|performance| performance.now());
            match filtered_data_rows(
                &self.data_index,
                &self.data_filter,
                self.data_filter_mode,
                self.data_descending,
            ) {
                Ok(rows) => {
                    self.data_rows = rows;
                    self.data_filter_error = None;
                    self.normalize_filtered_selection();
                }
                Err(error) => self.data_filter_error = Some(error),
            }
            self.data_filter_ms = started
                .and_then(|started| {
                    web_sys::window()
                        .and_then(|window| window.performance())
                        .map(|performance| performance.now() - started)
                })
                .unwrap_or(0.0);
        }

        fn normalize_filtered_selection(&mut self) {
            self.scroll = clamp_scroll_for(
                self.data_rows.len(),
                self.scroll,
                self.layout().list_content.height,
            );
            if !self.data_rows.contains(&self.selected)
                && let Some(row) = self.data_rows.first().copied()
            {
                self.select_list_row(row, false, false);
            }
        }

        fn sync_selection_from_input(&mut self) {
            self.selection_start = self.input.selection_start().ok().flatten().unwrap_or(0);
            self.selection_end = self
                .input
                .selection_end()
                .ok()
                .flatten()
                .unwrap_or(self.selection_start);
            self.selection_backward = self
                .input
                .selection_direction()
                .ok()
                .flatten()
                .is_some_and(|direction| direction == "backward");
            self.ensure_caret_visible();
        }

        fn active_caret(&self) -> u32 {
            if self.selection_backward {
                self.selection_start
            } else {
                self.selection_end
            }
        }

        fn apply_editor_navigation(&mut self, new_active: u32, extend: bool) {
            let new_active = new_active.min(utf16_len(&self.text));
            if extend {
                let anchor = if self.selection_backward {
                    self.selection_end
                } else {
                    self.selection_start
                };
                self.selection_start = anchor.min(new_active);
                self.selection_end = anchor.max(new_active);
                self.selection_backward = new_active < anchor;
            } else {
                self.selection_start = new_active;
                self.selection_end = new_active;
                self.selection_backward = false;
            }
            let direction = if self.selection_backward {
                "backward"
            } else {
                "forward"
            };
            let _ = self.input.set_selection_range_with_direction(
                self.selection_start,
                self.selection_end,
                direction,
            );
            self.ensure_caret_visible();
        }

        fn ensure_caret_visible(&mut self) {
            let viewport =
                (self.layout().text.height - EDITOR_PADDING * 2.0).max(EDITOR_LINE_HEIGHT);
            let caret_y =
                line_for_utf16(&self.text, self.active_caret()) as f64 * EDITOR_LINE_HEIGHT;
            if caret_y < self.editor_scroll {
                self.editor_scroll = caret_y;
            } else if caret_y + EDITOR_LINE_HEIGHT > self.editor_scroll + viewport {
                self.editor_scroll = caret_y + EDITOR_LINE_HEIGHT - viewport;
            }
            let content_height = text_line_count(&self.text) as f64 * EDITOR_LINE_HEIGHT;
            self.editor_scroll = self
                .editor_scroll
                .clamp(0.0, (content_height - viewport).max(0.0));
        }
    }

    fn handle_transcript_navigation(lab: &mut Lab, event: &KeyboardEvent) -> bool {
        if event.alt_key() || event.ctrl_key() || event.meta_key() {
            return false;
        }
        let key = event.key().to_ascii_lowercase();
        match key.as_str() {
            " " => {
                event.prevent_default();
                lab.transcript_playing = !lab.transcript_playing;
                return true;
            }
            "f" => {
                event.prevent_default();
                lab.transcript_follow = !lab.transcript_follow;
                return true;
            }
            _ => {}
        }
        if lab.transcript_pane == TranscriptPane::Speakers {
            let current = lab
                .transcript_speaker_filter
                .map_or(0, |speaker| speaker + 1);
            let position = match key.as_str() {
                "arrowdown" | "j" => (current + 1).min(SPEAKERS.len()),
                "arrowup" | "k" => current.saturating_sub(1),
                "home" | "h" => 0,
                "end" | "l" => SPEAKERS.len(),
                _ => return false,
            };
            event.prevent_default();
            lab.transcript_speaker_filter = (position > 0).then_some(position - 1);
            let rows = lab.transcript_rows();
            if let Some(segment) = rows.first().copied() {
                lab.select_transcript_segment(segment, false, false);
            }
            return true;
        }

        let rows = lab.transcript_rows();
        let current = rows
            .iter()
            .position(|index| *index == lab.transcript_selected)
            .unwrap_or(0);
        let position = match key.as_str() {
            "arrowdown" | "j" => (current + 1).min(rows.len().saturating_sub(1)),
            "arrowup" | "k" => current.saturating_sub(1),
            "home" | "h" => 0,
            "end" | "l" => rows.len().saturating_sub(1),
            _ => return false,
        };
        event.prevent_default();
        if let Some(segment) = rows.get(position).copied() {
            lab.select_transcript_segment(segment, event.shift_key(), false);
        }
        true
    }

    fn start_transcript_edit(app: &Rc<RefCell<Lab>>) {
        let (input, text) = {
            let mut lab = app.borrow_mut();
            let target = lab.transcript_selected;
            lab.transcript_playing = false;
            lab.transcript_edit_target = Some(target);
            lab.focus = Focus::Text;
            (
                lab.input.clone(),
                lab.transcript_segments[target].text.clone(),
            )
        };
        input.set_value(&text);
        let _ = input.remove_attribute("hidden");
        let _ = input.set_attribute("aria-label", "Edit transcript segment");
        let _ = input.focus();
        let end = utf16_len(&text);
        let _ = input.set_selection_range(end, end);
        position_text_proxy(app);
        invalidate(app);
    }

    fn stop_transcript_edit(app: &Rc<RefCell<Lab>>) {
        let (input, text, list) = {
            let mut lab = app.borrow_mut();
            lab.transcript_edit_target = None;
            lab.focus = Focus::List;
            (lab.input.clone(), lab.text.clone(), lab.a11y_list.clone())
        };
        input.set_value(&text);
        let _ = input.set_attribute("aria-label", "Selected transcript segment");
        let _ = list.focus();
        invalidate(app);
    }

    fn handle_diff_navigation(lab: &mut Lab, event: &KeyboardEvent) -> bool {
        if event.alt_key() || event.ctrl_key() || event.meta_key() {
            return false;
        }
        let key = event.key().to_ascii_lowercase();
        if lab.diff_pane == DiffPane::Files {
            let file = match key.as_str() {
                "arrowdown" | "j" => (lab.diff_file + 1).min(DIFF_FILES.len() - 1),
                "arrowup" | "k" => lab.diff_file.saturating_sub(1),
                "home" | "h" => 0,
                "end" | "l" => DIFF_FILES.len() - 1,
                _ => return false,
            };
            event.prevent_default();
            if file != lab.diff_file {
                lab.set_diff_file(file);
            }
            return true;
        }

        match key.as_str() {
            "arrowdown" | "j" => {
                event.prevent_default();
                lab.select_diff_row(lab.diff_selected + 1, event.shift_key());
            }
            "arrowup" | "k" => {
                event.prevent_default();
                lab.select_diff_row(lab.diff_selected.saturating_sub(1), event.shift_key());
            }
            "home" | "h" => {
                event.prevent_default();
                lab.select_diff_row(0, event.shift_key());
            }
            "end" | "l" => {
                event.prevent_default();
                let count = diff_display_rows(&lab.diff_lines, lab.diff_collapsed).len();
                lab.select_diff_row(count.saturating_sub(1), event.shift_key());
            }
            "n" => {
                event.prevent_default();
                lab.select_diff_change(true);
            }
            "p" => {
                event.prevent_default();
                lab.select_diff_change(false);
            }
            "c" => {
                event.prevent_default();
                lab.toggle_diff_collapse();
            }
            _ => return false,
        }
        true
    }

    fn handle_list_navigation(lab: &mut Lab, event: &KeyboardEvent) -> bool {
        let key = event.key();
        let modified = event.alt_key() || event.ctrl_key() || event.meta_key();
        if !modified && matches!(key.as_str(), "m" | "M" | "s" | "S" | "r" | "R") {
            event.prevent_default();
            match key.as_str() {
                "m" | "M" => {
                    lab.data_filter_mode = lab.data_filter_mode.next();
                    lab.refresh_data_rows();
                }
                "s" | "S" => {
                    lab.data_descending = !lab.data_descending;
                    lab.data_rows.reverse();
                    lab.normalize_filtered_selection();
                    lab.ensure_selected_visible();
                }
                _ => {
                    let selected = lab.selected_items.clone();
                    lab.reviewed_items.extend(selected);
                }
            }
            return true;
        }
        let current = lab
            .data_rows
            .iter()
            .position(|row| *row == lab.selected)
            .unwrap_or(0);
        let position = match key.as_str() {
            "ArrowDown" => (current + 1).min(lab.data_rows.len().saturating_sub(1)),
            "ArrowUp" => current.saturating_sub(1),
            "Home" => 0,
            "End" => lab.data_rows.len().saturating_sub(1),
            "j" | "J" if !modified => (current + 1).min(lab.data_rows.len().saturating_sub(1)),
            "k" | "K" if !modified => current.saturating_sub(1),
            "h" | "H" if !modified => 0,
            "l" | "L" if !modified => lab.data_rows.len().saturating_sub(1),
            _ => return false,
        };
        event.prevent_default();
        if let Some(row) = lab.data_rows.get(position).copied() {
            lab.select_list_row(row, event.shift_key(), false);
            lab.ensure_selected_visible();
        }
        true
    }

    fn schedule_data_filter(app: &Rc<RefCell<Lab>>) {
        let generation = {
            let mut lab = app.borrow_mut();
            if lab.data_filter_mode != FilterMode::Regex {
                lab.refresh_data_rows();
                drop(lab);
                invalidate(app);
                return;
            }
            lab.data_filter_generation = lab.data_filter_generation.wrapping_add(1);
            lab.data_filter_pending = true;
            lab.data_filter_generation
        };
        invalidate(app);

        let app = Rc::clone(app);
        let callback = Closure::once_into_js(move || {
            let mut lab = app.borrow_mut();
            if lab.data_filter_generation != generation {
                return;
            }
            lab.refresh_data_rows();
            drop(lab);
            invalidate(&app);
        });
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.unchecked_ref(),
                120,
            );
        }
    }

    #[wasm_bindgen(start)]
    pub fn start() -> Result<(), JsValue> {
        console_error_panic_hook::set_once();
        let window = web_sys::window().ok_or("window unavailable")?;
        let document = window.document().ok_or("document unavailable")?;
        let body = document.body().ok_or("body unavailable")?;

        let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
        canvas.set_tab_index(-1);
        canvas.set_attribute("aria-hidden", "true")?;
        let style = canvas.style();
        style.set_property("position", "fixed")?;
        style.set_property("inset", "0")?;
        style.set_property("width", "100%")?;
        style.set_property("height", "100%")?;
        style.set_property("display", "block")?;
        style.set_property("outline", "none")?;
        style.set_property("touch-action", "none")?;
        body.append_child(&canvas)?;

        let a11y_mirror: HtmlElement = document.create_element("div")?.dyn_into()?;
        a11y_mirror.set_class_name("a11y-mirror");

        let a11y_mode_document: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_mode_document.set_text_content(Some("Open Document and Data workflow"));
        a11y_mirror.append_child(&a11y_mode_document)?;

        let a11y_mode_diff: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_mode_diff.set_text_content(Some("Open Diff workflow"));
        a11y_mirror.append_child(&a11y_mode_diff)?;

        let a11y_mode_transcript: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_mode_transcript.set_text_content(Some("Open Transcript workflow"));
        a11y_mirror.append_child(&a11y_mode_transcript)?;

        let a11y_transcript_search: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_transcript_search.set_attribute("hidden", "")?;
        a11y_transcript_search.set_text_content(Some("Fuzzy search transcript"));
        a11y_mirror.append_child(&a11y_transcript_search)?;

        let a11y_transcript_play: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_transcript_play.set_attribute("hidden", "")?;
        a11y_transcript_play.set_text_content(Some("Play transcript"));
        a11y_mirror.append_child(&a11y_transcript_play)?;

        let a11y_transcript_follow: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_transcript_follow.set_attribute("hidden", "")?;
        a11y_transcript_follow.set_text_content(Some("Follow transcript playback"));
        a11y_mirror.append_child(&a11y_transcript_follow)?;

        let a11y_diff_previous: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_diff_previous.set_attribute("hidden", "")?;
        a11y_diff_previous.set_text_content(Some("Previous diff change"));
        a11y_mirror.append_child(&a11y_diff_previous)?;

        let a11y_diff_next: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_diff_next.set_attribute("hidden", "")?;
        a11y_diff_next.set_text_content(Some("Next diff change"));
        a11y_mirror.append_child(&a11y_diff_next)?;

        let a11y_diff_collapse: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_diff_collapse.set_attribute("hidden", "")?;
        a11y_diff_collapse.set_text_content(Some("Collapse unchanged diff lines"));
        a11y_mirror.append_child(&a11y_diff_collapse)?;

        let a11y_action: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_action.set_text_content(Some("Record action"));
        a11y_mirror.append_child(&a11y_action)?;

        let a11y_toggle: HtmlElement = document.create_element("button")?.dyn_into()?;
        a11y_toggle.set_attribute("role", "switch")?;
        a11y_toggle.set_attribute("aria-checked", "true")?;
        a11y_toggle.set_text_content(Some("Custom rendering"));
        a11y_mirror.append_child(&a11y_toggle)?;

        let input: HtmlTextAreaElement = document.create_element("textarea")?.dyn_into()?;
        input.set_attribute("aria-label", "Multiline editor")?;
        input.set_attribute("autocomplete", "off")?;
        input.set_attribute("autocapitalize", "off")?;
        input.set_value(DEFAULT_TEXT);
        let input_style = input.style();
        input_style.set_property("position", "fixed")?;
        input_style.set_property("width", "2px")?;
        input_style.set_property("height", "2px")?;
        input_style.set_property("opacity", "0")?;
        input_style.set_property("border", "0")?;
        input_style.set_property("padding", "0")?;
        input_style.set_property("resize", "none")?;
        input_style.set_property("pointer-events", "none")?;
        a11y_mirror.append_child(&input)?;

        let a11y_list: HtmlElement = document.create_element("div")?.dyn_into()?;
        a11y_list.set_tab_index(0);
        a11y_list.set_attribute("role", "listbox")?;
        a11y_list.set_attribute("aria-label", "Virtualized list")?;
        a11y_list.set_attribute("aria-multiselectable", "true")?;
        a11y_list.set_attribute("aria-activedescendant", "list-option-0")?;
        a11y_list.set_attribute("aria-describedby", "selection-status")?;

        let a11y_option: HtmlElement = document.create_element("div")?.dyn_into()?;
        a11y_option.set_id("list-option-0");
        a11y_option.set_attribute("role", "option")?;
        a11y_list.append_child(&a11y_option)?;
        a11y_mirror.append_child(&a11y_list)?;

        let a11y_status: HtmlElement = document.create_element("div")?.dyn_into()?;
        a11y_status.set_id("selection-status");
        a11y_status.set_attribute("role", "status")?;
        a11y_status.set_attribute("aria-live", "polite")?;
        a11y_mirror.append_child(&a11y_status)?;
        body.append_child(&a11y_mirror)?;

        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")?
            .ok_or("2D context unavailable")?
            .dyn_into()?;

        let app = Rc::new(RefCell::new(Lab {
            canvas,
            input,
            a11y_mode_document,
            a11y_mode_diff,
            a11y_mode_transcript,
            a11y_transcript_search,
            a11y_transcript_play,
            a11y_transcript_follow,
            a11y_diff_previous,
            a11y_diff_next,
            a11y_diff_collapse,
            a11y_action,
            a11y_toggle,
            a11y_list,
            a11y_option,
            a11y_status,
            context,
            mode: WorkbenchMode::DocumentData,
            width: 0.0,
            height: 0.0,
            dpr: 1.0,
            pointer_x: 0.0,
            pointer_y: 0.0,
            scroll: 0.0,
            selected: 0,
            selected_items: BTreeSet::from([0]),
            list_anchor: 0,
            action_count: 0,
            enabled: true,
            text: DEFAULT_TEXT.into(),
            composing: false,
            composition_anchor: None,
            selection_start: 0,
            selection_end: 0,
            selection_backward: false,
            drag_anchor: None,
            preferred_editor_x: None,
            delegated_native_navigation: false,
            editor_scroll: 0.0,
            overlay: Overlay::None,
            search_query: String::new(),
            active_match: 0,
            command_query: String::new(),
            command_selected: 0,
            data_filter: String::new(),
            data_filter_mode: FilterMode::Fuzzy,
            data_filter_error: None,
            data_filter_ms: 0.0,
            data_filter_generation: 0,
            data_filter_pending: false,
            data_filter_modal: true,
            data_descending: false,
            data_index: data_search_index(),
            data_rows: (0..ITEM_COUNT).collect(),
            reviewed_items: BTreeSet::new(),
            diff_file: 0,
            diff_lines: generate_diff(0, 5_000),
            diff_scroll: 0.0,
            diff_selected: 0,
            diff_anchor: 0,
            diff_selection_start: 0,
            diff_selection_end: 0,
            diff_collapsed: true,
            diff_pane: DiffPane::Files,
            transcript_segments: generate_transcript(2_000),
            transcript_scroll: 0.0,
            transcript_selected: 0,
            transcript_anchor: 0,
            transcript_selected_items: BTreeSet::from([0]),
            transcript_speaker_filter: None,
            transcript_pane: TranscriptPane::Speakers,
            transcript_query: String::new(),
            transcript_active_match: 0,
            transcript_playback: 0,
            transcript_playing: false,
            transcript_follow: true,
            transcript_edit_target: None,
            last_input: "none".into(),
            focus: Focus::Action,
            hover: None,
            render_pending: false,
            last_render_ms: 0.0,
        }));

        install_events(&app)?;
        resize(&app)?;
        let canvas = app.borrow().canvas.clone();
        canvas.focus()?;
        Ok(())
    }

    fn set_workbench_mode(app: &Rc<RefCell<Lab>>, mode: WorkbenchMode) {
        let (input, text, list) = {
            let mut lab = app.borrow_mut();
            lab.mode = mode;
            lab.overlay = Overlay::None;
            lab.hover = None;
            lab.transcript_edit_target = None;
            lab.focus = match mode {
                WorkbenchMode::Diff => {
                    lab.diff_pane = DiffPane::Files;
                    Focus::List
                }
                WorkbenchMode::Transcript => {
                    lab.transcript_pane = TranscriptPane::Speakers;
                    Focus::List
                }
                WorkbenchMode::DocumentData => Focus::Action,
            };
            (lab.input.clone(), lab.text.clone(), lab.a11y_list.clone())
        };
        app.borrow().sync_accessibility();
        if mode == WorkbenchMode::DocumentData {
            input.set_value(&text);
            let _ = input.set_attribute("aria-label", "Multiline editor");
        } else {
            input.set_value("");
        }
        let _ = input.remove_attribute("role");
        if matches!(mode, WorkbenchMode::Diff | WorkbenchMode::Transcript) {
            let _ = list.focus();
        } else {
            focus_accessibility(app, Focus::Action);
        }
        invalidate(app);
    }

    fn focus_listener(app: &Rc<RefCell<Lab>>, focus: Focus) -> Closure<dyn FnMut(Event)> {
        let app = Rc::clone(app);
        Closure::new(move |_: Event| {
            app.borrow_mut().focus = focus;
            invalidate(&app);
        })
    }

    fn install_events(app: &Rc<RefCell<Lab>>) -> Result<(), JsValue> {
        let canvas = app.borrow().canvas.clone();

        {
            let control = app.borrow().a11y_mode_document.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                set_workbench_mode(&app, WorkbenchMode::DocumentData);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let control = app.borrow().a11y_mode_diff.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                set_workbench_mode(&app, WorkbenchMode::Diff);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let control = app.borrow().a11y_mode_transcript.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                set_workbench_mode(&app, WorkbenchMode::Transcript);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let control = app.borrow().a11y_transcript_search.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                open_overlay(&app, Overlay::TranscriptSearch);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let control = app.borrow().a11y_transcript_play.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let playing = app.borrow().transcript_playing;
                app.borrow_mut().transcript_playing = !playing;
                invalidate(&app);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let control = app.borrow().a11y_transcript_follow.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let follow = app.borrow().transcript_follow;
                app.borrow_mut().transcript_follow = !follow;
                invalidate(&app);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let control = app.borrow().a11y_diff_previous.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let mut lab = app.borrow_mut();
                lab.diff_pane = DiffPane::Content;
                lab.select_diff_change(false);
                drop(lab);
                invalidate(&app);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let control = app.borrow().a11y_diff_next.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let mut lab = app.borrow_mut();
                lab.diff_pane = DiffPane::Content;
                lab.select_diff_change(true);
                drop(lab);
                invalidate(&app);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let control = app.borrow().a11y_diff_collapse.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let mut lab = app.borrow_mut();
                lab.diff_pane = DiffPane::Content;
                lab.toggle_diff_collapse();
                drop(lab);
                invalidate(&app);
            });
            control.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let action = app.borrow().a11y_action.clone();
            let closure = focus_listener(app, Focus::Action);
            action.add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())?;
            closure.forget();

            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let mut lab = app.borrow_mut();
                lab.focus = Focus::Action;
                lab.action_count += 1;
                drop(lab);
                invalidate(&app);
            });
            action.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let toggle = app.borrow().a11y_toggle.clone();
            let closure = focus_listener(app, Focus::Toggle);
            toggle.add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())?;
            closure.forget();

            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let mut lab = app.borrow_mut();
                lab.focus = Focus::Toggle;
                lab.enabled = !lab.enabled;
                drop(lab);
                invalidate(&app);
            });
            toggle.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let input = app.borrow().input.clone();
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let mut lab = app.borrow_mut();
                if lab.mode == WorkbenchMode::DocumentData && lab.overlay == Overlay::None {
                    lab.focus = Focus::Text;
                }
                drop(lab);
                invalidate(&app);
            });
            input.add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let list = app.borrow().a11y_list.clone();
            let closure = focus_listener(app, Focus::List);
            list.add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())?;
            closure.forget();

            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                let mut start_edit = false;
                let mut lab = app.borrow_mut();
                lab.focus = Focus::List;
                if lab.mode == WorkbenchMode::Diff {
                    handle_diff_navigation(&mut lab, &event);
                } else if lab.mode == WorkbenchMode::Transcript {
                    if event.key() == "Enter" && lab.transcript_pane == TranscriptPane::Segments {
                        event.prevent_default();
                        start_edit = true;
                    } else {
                        handle_transcript_navigation(&mut lab, &event);
                    }
                } else {
                    let handled = handle_list_navigation(&mut lab, &event);
                    if !handled && event.key() == " " {
                        event.prevent_default();
                        let row = lab.selected;
                        lab.select_list_row(row, event.shift_key(), !event.shift_key());
                    }
                }
                drop(lab);
                if start_edit {
                    start_transcript_edit(&app);
                } else {
                    invalidate(&app);
                }
            });
            list.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
                let rect = app.borrow().canvas.get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();
                let mut lab = app.borrow_mut();
                lab.pointer_x = x;
                lab.pointer_y = y;
                let next = lab.hover_at(x, y);
                let mut changed = next != lab.hover;
                lab.hover = next;
                if let Some(anchor) = lab.drag_anchor {
                    let position = text_position_at(
                        &lab.context,
                        &lab.text,
                        lab.layout().text,
                        lab.editor_scroll,
                        x,
                        y,
                    );
                    lab.selection_start = anchor.min(position);
                    lab.selection_end = anchor.max(position);
                    lab.selection_backward = position < anchor;
                    let direction = if lab.selection_backward {
                        "backward"
                    } else {
                        "forward"
                    };
                    let _ = lab.input.set_selection_range_with_direction(
                        lab.selection_start,
                        lab.selection_end,
                        direction,
                    );
                    lab.ensure_caret_visible();
                    changed = true;
                }
                let cursor = match next {
                    Some(Hover::Text) => "text",
                    Some(_) => "pointer",
                    None => "default",
                };
                let _ = lab.canvas.style().set_property("cursor", cursor);
                drop(lab);
                if changed {
                    position_text_proxy(&app);
                    invalidate(&app);
                }
            });
            canvas.add_event_listener_with_callback(
                "pointermove",
                closure.as_ref().unchecked_ref(),
            )?;
            // PROTOTYPE: page-lifetime listeners are leaked; production should own and remove them on teardown.
            closure.forget();
        }

        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
                event.prevent_default();
                let rect = app.borrow().canvas.get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();
                let overlay = app.borrow().overlay;
                if overlay != Overlay::None {
                    let inline_filter =
                        overlay == Overlay::DataFilter && !app.borrow().data_filter_modal;
                    let inside = if inline_filter {
                        app.borrow().layout().data_filter.contains(x, y)
                    } else {
                        overlay_rect(&app.borrow()).contains(x, y)
                    };
                    if inside {
                        if overlay != Overlay::Shortcuts {
                            let input = app.borrow().input.clone();
                            let _ = input.focus();
                        }
                    } else if overlay == Overlay::Shortcuts {
                        close_shortcuts(&app);
                    } else {
                        close_overlay(&app);
                    }
                    return;
                }
                let hit = app.borrow().hover_at(x, y);
                let transcript_interaction = matches!(
                    hit,
                    Some(
                        Hover::TranscriptSpeaker(_)
                            | Hover::TranscriptSegment(_)
                            | Hover::TranscriptPlay
                            | Hover::TranscriptFollow
                    )
                );
                let mut focus_text = false;
                let mut open_data_filter = false;
                let mut keep_data_filter_focus = false;
                let mut switch_mode = None;
                let mut start_transcript_search = false;
                {
                    let mut lab = app.borrow_mut();
                    lab.drag_anchor = None;
                    if transcript_interaction {
                        lab.transcript_edit_target = None;
                    }
                    match hit {
                        Some(Hover::ModeDocumentData) => {
                            switch_mode = Some(WorkbenchMode::DocumentData);
                        }
                        Some(Hover::ModeDiff) => {
                            switch_mode = Some(WorkbenchMode::Diff);
                        }
                        Some(Hover::ModeTranscript) => {
                            switch_mode = Some(WorkbenchMode::Transcript);
                        }
                        Some(Hover::TranscriptSpeaker(position)) => {
                            lab.transcript_pane = TranscriptPane::Speakers;
                            lab.transcript_speaker_filter = (position > 0).then_some(position - 1);
                            let rows = lab.transcript_rows();
                            if let Some(segment) = rows.first().copied() {
                                lab.select_transcript_segment(segment, false, false);
                            }
                        }
                        Some(Hover::TranscriptSegment(segment)) => {
                            lab.transcript_pane = TranscriptPane::Segments;
                            lab.select_transcript_segment(
                                segment,
                                event.shift_key(),
                                event.ctrl_key() || event.meta_key(),
                            );
                        }
                        Some(Hover::TranscriptSearch) => {
                            start_transcript_search = true;
                        }
                        Some(Hover::TranscriptPlay) => {
                            lab.transcript_playing = !lab.transcript_playing;
                        }
                        Some(Hover::TranscriptFollow) => {
                            lab.transcript_follow = !lab.transcript_follow;
                        }
                        Some(Hover::DiffFile(file)) => {
                            lab.diff_pane = DiffPane::Files;
                            lab.set_diff_file(file);
                        }
                        Some(Hover::DiffRow(row)) => {
                            lab.diff_pane = DiffPane::Content;
                            lab.select_diff_row(row, event.shift_key());
                        }
                        Some(Hover::DiffPrevious) => {
                            lab.diff_pane = DiffPane::Content;
                            lab.select_diff_change(false);
                        }
                        Some(Hover::DiffNext) => {
                            lab.diff_pane = DiffPane::Content;
                            lab.select_diff_change(true);
                        }
                        Some(Hover::DiffCollapse) => {
                            lab.diff_pane = DiffPane::Content;
                            lab.toggle_diff_collapse();
                        }
                        Some(Hover::Action) => {
                            lab.focus = Focus::Action;
                            lab.action_count += 1;
                        }
                        Some(Hover::Toggle) => {
                            lab.focus = Focus::Toggle;
                            lab.enabled = !lab.enabled;
                        }
                        Some(Hover::Text) => {
                            lab.focus = Focus::Text;
                            let position = text_position_at(
                                &lab.context,
                                &lab.text,
                                lab.layout().text,
                                lab.editor_scroll,
                                x,
                                y,
                            );
                            lab.selection_start = position;
                            lab.selection_end = position;
                            lab.selection_backward = false;
                            lab.preferred_editor_x = None;
                            lab.drag_anchor = Some(position);
                            let _ = lab
                                .input
                                .set_selection_range_with_direction(position, position, "forward");
                            let _ = lab.canvas.set_pointer_capture(event.pointer_id());
                            focus_text = true;
                        }
                        Some(Hover::DataFilter) => {
                            lab.focus = Focus::List;
                            open_data_filter = true;
                        }
                        Some(Hover::DataFilterClear) => {
                            lab.focus = Focus::List;
                            lab.data_filter.clear();
                            keep_data_filter_focus = lab.overlay == Overlay::DataFilter;
                            if keep_data_filter_focus {
                                lab.input.set_value("");
                            }
                            lab.refresh_data_rows();
                        }
                        Some(Hover::DataFilterMode) => {
                            lab.focus = Focus::List;
                            lab.data_filter_mode = lab.data_filter_mode.next();
                            lab.refresh_data_rows();
                        }
                        Some(Hover::DataSort) => {
                            lab.focus = Focus::List;
                            lab.data_descending = !lab.data_descending;
                            lab.data_rows.reverse();
                            lab.normalize_filtered_selection();
                            lab.ensure_selected_visible();
                        }
                        Some(Hover::DataBatch) => {
                            lab.focus = Focus::List;
                            let selected = lab.selected_items.clone();
                            lab.reviewed_items.extend(selected);
                        }
                        Some(Hover::Row(row)) => {
                            lab.focus = Focus::List;
                            lab.drag_anchor = None;
                            lab.select_list_row(
                                row,
                                event.shift_key(),
                                event.ctrl_key() || event.meta_key(),
                            );
                        }
                        None => {}
                    }
                }
                if let Some(mode) = switch_mode {
                    set_workbench_mode(&app, mode);
                } else if start_transcript_search {
                    open_overlay(&app, Overlay::TranscriptSearch);
                } else if focus_text {
                    focus_text_proxy(&app);
                } else if open_data_filter {
                    open_data_filter_input(&app, false);
                } else if keep_data_filter_focus {
                    let input = app.borrow().input.clone();
                    let _ = input.focus();
                    let _ = input.set_selection_range(0, 0);
                    position_text_proxy(&app);
                } else if transcript_interaction {
                    let list = app.borrow().a11y_list.clone();
                    let _ = list.focus();
                } else {
                    let canvas = app.borrow().canvas.clone();
                    let _ = canvas.focus();
                }
                invalidate(&app);
            });
            canvas.add_event_listener_with_callback(
                "pointerdown",
                closure.as_ref().unchecked_ref(),
            )?;
            closure.forget();
        }

        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
                let mut lab = app.borrow_mut();
                lab.drag_anchor = None;
                let _ = lab.canvas.release_pointer_capture(event.pointer_id());
            });
            canvas
                .add_event_listener_with_callback("pointerup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(WheelEvent)>::new(move |event: WheelEvent| {
                if app.borrow().overlay != Overlay::None {
                    event.prevent_default();
                    return;
                }
                let rect = app.borrow().canvas.get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();
                let mut lab = app.borrow_mut();
                if lab.mode == WorkbenchMode::Transcript {
                    let rows_rect = transcript_rows_rect(lab.transcript_layout());
                    if !rows_rect.contains(x, y) {
                        return;
                    }
                    event.prevent_default();
                    let delta = match event.delta_mode() {
                        1 => event.delta_y() * ROW_HEIGHT,
                        2 => event.delta_y() * rows_rect.height,
                        _ => event.delta_y(),
                    };
                    let count = lab.transcript_rows().len();
                    lab.transcript_scroll =
                        clamp_scroll_for(count, lab.transcript_scroll + delta, rows_rect.height);
                    drop(lab);
                    invalidate(&app);
                    return;
                }
                if lab.mode == WorkbenchMode::Diff {
                    let layout = lab.diff_layout();
                    let rows_rect = diff_rows_rect(layout);
                    if !rows_rect.contains(x, y) {
                        return;
                    }
                    event.prevent_default();
                    let viewport = rows_rect.height;
                    let delta = match event.delta_mode() {
                        1 => event.delta_y() * ROW_HEIGHT,
                        2 => event.delta_y() * viewport,
                        _ => event.delta_y(),
                    };
                    let count = diff_display_rows(&lab.diff_lines, lab.diff_collapsed).len();
                    lab.diff_scroll = clamp_scroll_for(count, lab.diff_scroll + delta, viewport);
                    drop(lab);
                    invalidate(&app);
                    return;
                }
                let layout = lab.layout();
                let viewport = if layout.list.contains(x, y) {
                    layout.list_content.height
                } else if layout.text.contains(x, y) {
                    (layout.text.height - EDITOR_PADDING * 2.0).max(EDITOR_LINE_HEIGHT)
                } else {
                    return;
                };
                event.prevent_default();
                lab.pointer_x = x;
                lab.pointer_y = y;
                let delta = match event.delta_mode() {
                    1 => event.delta_y() * ROW_HEIGHT,
                    2 => event.delta_y() * viewport,
                    _ => event.delta_y(),
                };
                if layout.text.contains(x, y) {
                    let content = text_line_count(&lab.text) as f64 * EDITOR_LINE_HEIGHT;
                    lab.editor_scroll =
                        (lab.editor_scroll + delta).clamp(0.0, (content - viewport).max(0.0));
                } else {
                    lab.scroll =
                        clamp_scroll_for(lab.data_rows.len(), lab.scroll + delta, viewport);
                }
                drop(lab);
                position_text_proxy(&app);
                invalidate(&app);
            });
            let options = AddEventListenerOptions::new();
            options.set_passive(false);
            canvas.add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                closure.as_ref().unchecked_ref(),
                &options,
            )?;
            closure.forget();
        }

        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                let key = event.key();
                let mut focus_target = None;
                let mut start_edit = false;
                {
                    let mut lab = app.borrow_mut();
                    if lab.mode == WorkbenchMode::Diff {
                        handle_diff_navigation(&mut lab, &event);
                        drop(lab);
                        invalidate(&app);
                        return;
                    }
                    if lab.mode == WorkbenchMode::Transcript {
                        if key == "Enter" && lab.transcript_pane == TranscriptPane::Segments {
                            event.prevent_default();
                            start_edit = true;
                        } else {
                            handle_transcript_navigation(&mut lab, &event);
                        }
                        drop(lab);
                        if start_edit {
                            start_transcript_edit(&app);
                        } else {
                            invalidate(&app);
                        }
                        return;
                    }
                    match key.as_str() {
                        "Tab" => {
                            event.prevent_default();
                            lab.focus = if event.shift_key() {
                                match lab.focus {
                                    Focus::Action => Focus::List,
                                    Focus::Toggle => Focus::Action,
                                    Focus::Text => Focus::Toggle,
                                    Focus::List => Focus::Text,
                                }
                            } else {
                                match lab.focus {
                                    Focus::Action => Focus::Toggle,
                                    Focus::Toggle => Focus::Text,
                                    Focus::Text => Focus::List,
                                    Focus::List => Focus::Action,
                                }
                            };
                            focus_target = Some(lab.focus);
                        }
                        "Enter" | " " if lab.focus == Focus::Action => {
                            event.prevent_default();
                            lab.action_count += 1;
                        }
                        "Enter" | " " if lab.focus == Focus::Toggle => {
                            event.prevent_default();
                            lab.enabled = !lab.enabled;
                        }
                        _ if lab.focus == Focus::List
                            && handle_list_navigation(&mut lab, &event) => {}
                        _ => {}
                    }
                }
                if let Some(focus) = focus_target {
                    focus_accessibility(&app, focus);
                }
                invalidate(&app);
            });
            canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        let input = app.borrow().input.clone();
        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(InputEvent)>::new(move |event: InputEvent| {
                app.borrow_mut().last_input = format!("beforeinput: {}", event.input_type());
                invalidate(&app);
            });
            input.add_event_listener_with_callback(
                "beforeinput",
                closure.as_ref().unchecked_ref(),
            )?;
            closure.forget();
        }
        {
            let app = Rc::clone(app);
            let input_for_event = input.clone();
            let closure = Closure::<dyn FnMut(InputEvent)>::new(move |event: InputEvent| {
                let mut lab = app.borrow_mut();
                let mut data_filter_changed = false;
                let value = input_for_event.value();
                match lab.overlay {
                    Overlay::None if lab.mode == WorkbenchMode::DocumentData => {
                        lab.text = value;
                        lab.preferred_editor_x = None;
                        lab.sync_selection_from_input();
                    }
                    Overlay::None if lab.mode == WorkbenchMode::Transcript => {
                        if let Some(target) = lab.transcript_edit_target {
                            lab.transcript_segments[target].text = value;
                        }
                    }
                    Overlay::None => {}
                    Overlay::Search => {
                        lab.search_query = value;
                        lab.active_match = 0;
                        select_search_match(&mut lab, 0);
                    }
                    Overlay::Command => {
                        lab.command_query = value;
                        lab.command_selected = 0;
                    }
                    Overlay::DataFilter => {
                        lab.data_filter = value;
                        data_filter_changed = true;
                    }
                    Overlay::TranscriptSearch => {
                        lab.transcript_query = value;
                        lab.transcript_active_match = 0;
                        lab.select_transcript_match(0);
                    }
                    Overlay::Shortcuts => {}
                }
                lab.last_input = format!("input: {}", event.input_type());
                drop(lab);
                if data_filter_changed {
                    schedule_data_filter(&app);
                }
                position_text_proxy(&app);
                invalidate(&app);
            });
            input.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let mut lab = app.borrow_mut();
                if lab.overlay == Overlay::None && lab.mode == WorkbenchMode::DocumentData {
                    lab.sync_selection_from_input();
                }
                drop(lab);
                position_text_proxy(&app);
                invalidate(&app);
            });
            input.add_event_listener_with_callback("select", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        for (name, composing) in [("compositionstart", true), ("compositionend", false)] {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let mut lab = app.borrow_mut();
                lab.composing = composing;
                if lab.overlay == Overlay::None && lab.mode == WorkbenchMode::DocumentData {
                    lab.sync_selection_from_input();
                    if composing {
                        lab.composition_anchor = Some(lab.active_caret());
                    } else {
                        lab.composition_anchor = None;
                    }
                }
                lab.last_input = if composing {
                    "compositionstart".into()
                } else {
                    "compositionend".into()
                };
                drop(lab);
                position_text_proxy(&app);
                invalidate(&app);
            });
            input.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let app = Rc::clone(app);
            let closure =
                Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                    if event.is_composing() || app.borrow().composing {
                        return;
                    }
                    let key = event.key();
                    let overlay = app.borrow().overlay;
                    if overlay == Overlay::None
                        && app.borrow().mode == WorkbenchMode::Transcript
                        && app.borrow().transcript_edit_target.is_some()
                    {
                        if key == "Escape" || (key == "Enter" && event.ctrl_key()) {
                            event.prevent_default();
                            stop_transcript_edit(&app);
                        }
                        return;
                    }
                    match overlay {
                        Overlay::Search => {
                            match key.as_str() {
                                "Escape" => {
                                    event.prevent_default();
                                    close_overlay(&app);
                                }
                                "Enter" | "F3" => {
                                    event.prevent_default();
                                    select_search_match(
                                        &mut app.borrow_mut(),
                                        if event.shift_key() { -1 } else { 1 },
                                    );
                                    invalidate(&app);
                                }
                                _ => {}
                            }
                            return;
                        }
                        Overlay::Command => {
                            match key.as_str() {
                                "Escape" => {
                                    event.prevent_default();
                                    close_overlay(&app);
                                }
                                "ArrowDown" | "ArrowUp" => {
                                    event.prevent_default();
                                    let mut lab = app.borrow_mut();
                                    let count = matching_commands(&lab.command_query).len();
                                    if count > 0 {
                                        lab.command_selected = if key == "ArrowDown" {
                                            (lab.command_selected + 1) % count
                                        } else {
                                            (lab.command_selected + count - 1) % count
                                        };
                                    }
                                    drop(lab);
                                    invalidate(&app);
                                }
                                "Enter" => {
                                    event.prevent_default();
                                    execute_command(&app);
                                }
                                _ => {}
                            }
                            return;
                        }
                        Overlay::TranscriptSearch => {
                            match key.as_str() {
                                "Escape" => {
                                    event.prevent_default();
                                    close_overlay(&app);
                                }
                                "Enter" | "F3" => {
                                    event.prevent_default();
                                    app.borrow_mut().select_transcript_match(
                                        if event.shift_key() { -1 } else { 1 },
                                    );
                                    invalidate(&app);
                                }
                                _ => {}
                            }
                            return;
                        }
                        Overlay::DataFilter => {
                            match key.as_str() {
                                "Escape" | "Enter" => {
                                    event.prevent_default();
                                    close_overlay(&app);
                                }
                                "Home" | "End" => {
                                    event.prevent_default();
                                    let input = app.borrow().input.clone();
                                    let target = if key == "Home" {
                                        0
                                    } else {
                                        utf16_len(&input.value())
                                    };
                                    move_input_to_boundary(&input, target, event.shift_key());
                                    invalidate(&app);
                                }
                                _ => {}
                            }
                            return;
                        }
                        Overlay::Shortcuts => return,
                        Overlay::None => {}
                    }

                    if key == "Escape" {
                        event.prevent_default();
                        app.borrow_mut().focus = Focus::List;
                        focus_accessibility(&app, Focus::List);
                        invalidate(&app);
                        return;
                    }

                    // Modified arrows retain the textarea's native word/document movement.
                    // Their final selection is synchronized by the keyup listener below.
                    if event.alt_key() || event.ctrl_key() || event.meta_key() {
                        if matches!(
                            key.as_str(),
                            "ArrowLeft"
                                | "ArrowRight"
                                | "ArrowUp"
                                | "ArrowDown"
                                | "Home"
                                | "End"
                                | "PageUp"
                                | "PageDown"
                        ) {
                            app.borrow_mut().delegated_native_navigation = true;
                        }
                        return;
                    }
                    let handled = navigate_editor(&mut app.borrow_mut(), &key, event.shift_key());
                    if handled {
                        event.prevent_default();
                        position_text_proxy(&app);
                        invalidate(&app);
                    }
                });
            input.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                if event.is_composing() || app.borrow().composing {
                    return;
                }
                if app.borrow().overlay != Overlay::None {
                    invalidate(&app);
                    return;
                }
                let mut lab = app.borrow_mut();
                if lab.delegated_native_navigation {
                    lab.delegated_native_navigation = false;
                    lab.sync_selection_from_input();
                    lab.preferred_editor_x = None;
                    drop(lab);
                    position_text_proxy(&app);
                    invalidate(&app);
                }
            });
            input.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                if event.is_composing() || app.borrow().composing {
                    return;
                }
                let overlay = app.borrow().overlay;
                if (event.ctrl_key() || event.meta_key()) && !event.alt_key() {
                    match event.key().as_str() {
                        "1" => {
                            event.prevent_default();
                            set_workbench_mode(&app, WorkbenchMode::DocumentData);
                            return;
                        }
                        "3" => {
                            event.prevent_default();
                            set_workbench_mode(&app, WorkbenchMode::Diff);
                            return;
                        }
                        "4" => {
                            event.prevent_default();
                            set_workbench_mode(&app, WorkbenchMode::Transcript);
                            return;
                        }
                        _ => {}
                    }
                }
                if overlay == Overlay::None
                    && event.alt_key()
                    && !(event.ctrl_key() || event.meta_key())
                {
                    let key = event.key().to_ascii_lowercase();
                    if matches!(key.as_str(), "h" | "j" | "k" | "l") {
                        if app.borrow().mode == WorkbenchMode::Transcript {
                            let next = match key.as_str() {
                                "h" => Some(TranscriptPane::Speakers),
                                "l" => Some(TranscriptPane::Segments),
                                _ => None,
                            };
                            if let Some(next) = next {
                                event.prevent_default();
                                app.borrow_mut().transcript_pane = next;
                                let list = app.borrow().a11y_list.clone();
                                let _ = list.focus();
                                invalidate(&app);
                            }
                        } else if app.borrow().mode == WorkbenchMode::Diff {
                            let next = match key.as_str() {
                                "h" => Some(DiffPane::Files),
                                "l" => Some(DiffPane::Content),
                                _ => None,
                            };
                            if let Some(next) = next {
                                event.prevent_default();
                                app.borrow_mut().diff_pane = next;
                                let list = app.borrow().a11y_list.clone();
                                let _ = list.focus();
                                invalidate(&app);
                            }
                        } else {
                            let current = app.borrow().focus;
                            let next = directional_focus(current, &key);
                            if next != current {
                                event.prevent_default();
                                app.borrow_mut().focus = next;
                                focus_accessibility(&app, next);
                                invalidate(&app);
                            }
                        }
                        return;
                    }
                }
                if overlay == Overlay::Shortcuts && event.key() == "Escape" {
                    event.prevent_default();
                    close_shortcuts(&app);
                    return;
                }
                if overlay == Overlay::None
                    && app.borrow().focus != Focus::Text
                    && !(event.alt_key() || event.ctrl_key() || event.meta_key())
                {
                    match event.key().as_str() {
                        "?" => {
                            event.prevent_default();
                            app.borrow_mut().overlay = Overlay::Shortcuts;
                            invalidate(&app);
                            return;
                        }
                        "/" if app.borrow().mode == WorkbenchMode::Transcript => {
                            event.prevent_default();
                            open_overlay(&app, Overlay::TranscriptSearch);
                            return;
                        }
                        "/" if app.borrow().mode == WorkbenchMode::DocumentData
                            && app.borrow().focus == Focus::List =>
                        {
                            event.prevent_default();
                            open_data_filter_input(&app, true);
                            return;
                        }
                        _ => {}
                    }
                }
                if !(event.ctrl_key() || event.meta_key()) {
                    return;
                }
                match event.key().to_ascii_lowercase().as_str() {
                    "f" if app.borrow().mode == WorkbenchMode::DocumentData => {
                        event.prevent_default();
                        let overlay = if app.borrow().focus == Focus::List {
                            Overlay::DataFilter
                        } else {
                            Overlay::Search
                        };
                        if overlay == Overlay::DataFilter {
                            open_data_filter_input(&app, true);
                        } else {
                            open_overlay(&app, overlay);
                        }
                    }
                    "p" if app.borrow().mode == WorkbenchMode::DocumentData => {
                        event.prevent_default();
                        open_overlay(&app, Overlay::Command);
                    }
                    _ => {}
                }
            });
            web_sys::window()
                .ok_or("window unavailable")?
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut()>::new(move || {
                let mut lab = app.borrow_mut();
                if lab.mode == WorkbenchMode::Transcript && lab.transcript_playing {
                    lab.advance_transcript_playback();
                    drop(lab);
                    invalidate(&app);
                }
            });
            web_sys::window()
                .ok_or("window unavailable")?
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    700,
                )?;
            closure.forget();
        }

        {
            let app = Rc::clone(app);
            let closure = Closure::<dyn FnMut(Event)>::new(move |_: Event| {
                let _ = resize(&app);
            });
            web_sys::window()
                .ok_or("window unavailable")?
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        Ok(())
    }

    fn overlay_rect(lab: &Lab) -> Rect {
        let preferred_width: f64 = match lab.overlay {
            Overlay::Command => 480.0,
            Overlay::Shortcuts => 520.0,
            _ => 360.0,
        };
        let width = preferred_width.min((lab.width - 32.0).max(240.0));
        Rect {
            x: (lab.width - width) / 2.0,
            y: if lab.overlay == Overlay::Command {
                112.0
            } else {
                76.0
            },
            width,
            height: match lab.overlay {
                Overlay::Command => 190.0,
                Overlay::Shortcuts => {
                    ((SHORTCUTS.len() as f64 * 32.0) + 82.0).min((lab.height - 100.0).max(220.0))
                }
                _ => 64.0,
            },
        }
    }

    fn open_data_filter_input(app: &Rc<RefCell<Lab>>, modal: bool) {
        app.borrow_mut().data_filter_modal = modal;
        open_overlay(app, Overlay::DataFilter);
    }

    fn open_overlay(app: &Rc<RefCell<Lab>>, overlay: Overlay) {
        let (input, value, label) = {
            let mut lab = app.borrow_mut();
            lab.overlay = overlay;
            if overlay == Overlay::TranscriptSearch {
                lab.transcript_edit_target = None;
            }
            lab.command_selected = 0;
            let value = match overlay {
                Overlay::Search => lab.search_query.clone(),
                Overlay::Command => lab.command_query.clone(),
                Overlay::DataFilter => lab.data_filter.clone(),
                Overlay::TranscriptSearch => lab.transcript_query.clone(),
                Overlay::Shortcuts => unreachable!(),
                Overlay::None => lab.text.clone(),
            };
            let label = match overlay {
                Overlay::Search => "Search document",
                Overlay::Command => "Command palette",
                Overlay::DataFilter => "Filter data rows",
                Overlay::TranscriptSearch => "Fuzzy search transcript",
                Overlay::Shortcuts => unreachable!(),
                Overlay::None => "Multiline editor",
            };
            (lab.input.clone(), value, label)
        };
        input.set_value(&value);
        let _ = input.remove_attribute("hidden");
        let _ = input.set_attribute("aria-label", label);
        let _ = input.set_attribute(
            "role",
            if matches!(
                overlay,
                Overlay::Search | Overlay::DataFilter | Overlay::TranscriptSearch
            ) {
                "searchbox"
            } else {
                "combobox"
            },
        );
        let end = utf16_len(&value);
        let _ = input.focus();
        let _ = input.set_selection_range(end, end);
        position_text_proxy(app);
        invalidate(app);
    }

    fn close_shortcuts(app: &Rc<RefCell<Lab>>) {
        let focus = {
            let mut lab = app.borrow_mut();
            lab.overlay = Overlay::None;
            lab.focus
        };
        focus_accessibility(app, focus);
        invalidate(app);
    }

    fn close_overlay(app: &Rc<RefCell<Lab>>) {
        let (input, text, start, end, direction, return_to_list) = {
            let mut lab = app.borrow_mut();
            let return_to_list =
                matches!(lab.overlay, Overlay::DataFilter | Overlay::TranscriptSearch);
            lab.overlay = Overlay::None;
            if return_to_list {
                lab.focus = Focus::List;
            }
            (
                lab.input.clone(),
                lab.text.clone(),
                lab.selection_start,
                lab.selection_end,
                if lab.selection_backward {
                    "backward"
                } else {
                    "forward"
                },
                return_to_list,
            )
        };
        input.set_value(&text);
        let _ = input.set_attribute("aria-label", "Multiline editor");
        let _ = input.remove_attribute("role");
        if return_to_list {
            focus_accessibility(app, Focus::List);
        } else {
            let _ = input.focus();
            let _ = input.set_selection_range_with_direction(start, end, direction);
            position_text_proxy(app);
        }
        invalidate(app);
    }

    fn move_input_to_boundary(input: &HtmlTextAreaElement, target: u32, extend: bool) {
        let start = input.selection_start().ok().flatten().unwrap_or(0);
        let end = input.selection_end().ok().flatten().unwrap_or(start);
        if !extend {
            let _ = input.set_selection_range_with_direction(target, target, "forward");
            return;
        }
        let backward = input
            .selection_direction()
            .ok()
            .flatten()
            .is_some_and(|direction| direction == "backward");
        let anchor = if backward { end } else { start };
        let direction = if target < anchor {
            "backward"
        } else {
            "forward"
        };
        let _ = input.set_selection_range_with_direction(
            anchor.min(target),
            anchor.max(target),
            direction,
        );
    }

    fn select_search_match(lab: &mut Lab, delta: isize) {
        let matches = find_utf16_matches(&lab.text, &lab.search_query);
        if matches.is_empty() {
            lab.active_match = 0;
            return;
        }
        lab.active_match =
            (lab.active_match as isize + delta).rem_euclid(matches.len() as isize) as usize;
        (lab.selection_start, lab.selection_end) = matches[lab.active_match];
        lab.selection_backward = false;
        lab.ensure_caret_visible();
    }

    fn execute_command(app: &Rc<RefCell<Lab>>) {
        {
            let mut lab = app.borrow_mut();
            let commands = matching_commands(&lab.command_query);
            let Some(command) = commands.get(lab.command_selected).copied() else {
                return;
            };
            match command {
                0 => {
                    lab.selection_start = 0;
                    lab.selection_end = utf16_len(&lab.text);
                    lab.selection_backward = false;
                    lab.ensure_caret_visible();
                }
                1 => {
                    lab.search_query.clear();
                    lab.active_match = 0;
                }
                2 => lab.enabled = !lab.enabled,
                _ => unreachable!(),
            }
        }
        close_overlay(app);
    }

    fn focus_accessibility(app: &Rc<RefCell<Lab>>, focus: Focus) {
        if focus == Focus::Text {
            focus_text_proxy(app);
            return;
        }
        let element = {
            let lab = app.borrow();
            match focus {
                Focus::Action => lab.a11y_action.clone(),
                Focus::Toggle => lab.a11y_toggle.clone(),
                Focus::List => lab.a11y_list.clone(),
                Focus::Text => unreachable!(),
            }
        };
        let _ = element.focus();
    }

    fn focus_text_proxy(app: &Rc<RefCell<Lab>>) {
        let (input, start, end, direction) = {
            let lab = app.borrow();
            (
                lab.input.clone(),
                lab.selection_start,
                lab.selection_end,
                if lab.selection_backward {
                    "backward"
                } else {
                    "forward"
                },
            )
        };
        let _ = input.focus();
        let _ = input.set_selection_range_with_direction(start, end, direction);
        position_text_proxy(app);
    }

    fn position_text_proxy(app: &Rc<RefCell<Lab>>) {
        let lab = app.borrow();
        let (x, y) = if lab.overlay == Overlay::None
            && lab.mode == WorkbenchMode::Transcript
            && lab.transcript_edit_target.is_some()
        {
            let target = lab
                .transcript_edit_target
                .unwrap_or(lab.transcript_selected);
            let rows = lab.transcript_rows();
            let position = rows.iter().position(|index| *index == target).unwrap_or(0);
            let viewport = transcript_rows_rect(lab.transcript_layout());
            (
                viewport.x + 120.0,
                viewport.y + position as f64 * ROW_HEIGHT - lab.transcript_scroll,
            )
        } else if lab.overlay == Overlay::None {
            caret_point(
                &lab.context,
                &lab.text,
                lab.active_caret(),
                lab.layout().text,
                lab.editor_scroll,
            )
        } else if lab.overlay == Overlay::DataFilter && !lab.data_filter_modal {
            let filter = lab.layout().data_filter;
            (filter.x + 8.0, filter.y - 8.0)
        } else {
            let overlay = overlay_rect(&lab);
            (overlay.x + 18.0, overlay.y + 9.0)
        };
        let style = lab.input.style();
        let _ = style.set_property("left", &format!("{}px", x));
        let _ = style.set_property("top", &format!("{}px", y + EDITOR_LINE_HEIGHT));
    }

    fn navigate_editor(lab: &mut Lab, key: &str, extend: bool) -> bool {
        let active = lab.active_caret();
        let has_selection = lab.selection_start != lab.selection_end;
        let editor = lab.layout().text;
        let new_active = match key {
            "ArrowLeft" => {
                lab.preferred_editor_x = None;
                if has_selection && !extend {
                    lab.selection_start
                } else {
                    previous_grapheme_utf16(&lab.text, active)
                }
            }
            "ArrowRight" => {
                lab.preferred_editor_x = None;
                if has_selection && !extend {
                    lab.selection_end
                } else {
                    next_grapheme_utf16(&lab.text, active)
                }
            }
            "Home" => {
                lab.preferred_editor_x = None;
                line_bounds_for_utf16(&lab.text, active).0
            }
            "End" => {
                lab.preferred_editor_x = None;
                line_bounds_for_utf16(&lab.text, active).1
            }
            "ArrowUp" | "ArrowDown" | "PageUp" | "PageDown" => {
                let (caret_x, caret_y) =
                    caret_point(&lab.context, &lab.text, active, editor, lab.editor_scroll);
                let target_x = *lab.preferred_editor_x.get_or_insert(caret_x);
                let page_lines = ((editor.height - EDITOR_PADDING * 2.0) / EDITOR_LINE_HEIGHT)
                    .floor()
                    .max(1.0);
                let line_delta = match key {
                    "ArrowUp" => -1.0,
                    "ArrowDown" => 1.0,
                    "PageUp" => -page_lines,
                    _ => page_lines,
                };
                text_position_at(
                    &lab.context,
                    &lab.text,
                    editor,
                    lab.editor_scroll,
                    target_x,
                    caret_y + line_delta * EDITOR_LINE_HEIGHT + EDITOR_LINE_HEIGHT / 2.0,
                )
            }
            _ => return false,
        };
        lab.apply_editor_navigation(new_active, extend);
        lab.last_input = format!("navigation: {key}");
        true
    }

    fn line_at(text: &str, line_index: usize) -> (u32, &str) {
        let mut start = 0;
        for (index, line) in text.split('\n').enumerate() {
            if index == line_index {
                return (start, line);
            }
            start += utf16_len(line) + 1;
        }
        (utf16_len(text), "")
    }

    fn text_position_at(
        ctx: &CanvasRenderingContext2d,
        text: &str,
        editor: Rect,
        scroll: f64,
        x: f64,
        y: f64,
    ) -> u32 {
        ctx.set_font(EDITOR_FONT);
        let line_count = text_line_count(text);
        let line_index = (((y - editor.y - EDITOR_PADDING + scroll) / EDITOR_LINE_HEIGHT)
            .floor()
            .max(0.0) as usize)
            .min(line_count - 1);
        let (line_start, line) = line_at(text, line_index);
        let target = (x - editor.x - EDITOR_PADDING).max(0.0);
        let mut width = 0.0;
        let mut offset = line_start;
        for grapheme in line.graphemes(true) {
            let grapheme_width = ctx.measure_text(grapheme).map(|m| m.width()).unwrap_or(0.0);
            if target < width + grapheme_width / 2.0 {
                return offset;
            }
            width += grapheme_width;
            offset += utf16_len(grapheme);
        }
        offset
    }

    fn caret_point(
        ctx: &CanvasRenderingContext2d,
        text: &str,
        offset: u32,
        editor: Rect,
        scroll: f64,
    ) -> (f64, f64) {
        ctx.set_font(EDITOR_FONT);
        let line_index = line_for_utf16(text, offset);
        let (line_start, line) = line_at(text, line_index);
        let local = offset.saturating_sub(line_start).min(utf16_len(line));
        let byte = utf16_to_byte(line, local);
        let width = ctx
            .measure_text(&line[..byte])
            .map(|metrics| metrics.width())
            .unwrap_or(0.0);
        (
            editor.x + EDITOR_PADDING + width,
            editor.y + EDITOR_PADDING + line_index as f64 * EDITOR_LINE_HEIGHT - scroll,
        )
    }

    fn resize(app: &Rc<RefCell<Lab>>) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("window unavailable")?;
        let width = window.inner_width()?.as_f64().ok_or("invalid width")?;
        let height = window.inner_height()?.as_f64().ok_or("invalid height")?;
        let dpr = window.device_pixel_ratio().clamp(1.0, 3.0);
        {
            let mut lab = app.borrow_mut();
            lab.width = width;
            lab.height = height;
            lab.dpr = dpr;
            lab.canvas.set_width((width * dpr).round() as u32);
            lab.canvas.set_height((height * dpr).round() as u32);
            lab.context.set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0)?;
            let viewport = lab.layout().list_content.height;
            lab.scroll = clamp_scroll_for(lab.data_rows.len(), lab.scroll, viewport);
            let diff_viewport = diff_rows_rect(lab.diff_layout()).height;
            let diff_count = diff_display_rows(&lab.diff_lines, lab.diff_collapsed).len();
            lab.diff_scroll = clamp_scroll_for(diff_count, lab.diff_scroll, diff_viewport);
            let transcript_viewport = transcript_rows_rect(lab.transcript_layout()).height;
            let transcript_count = lab.transcript_rows().len();
            lab.transcript_scroll =
                clamp_scroll_for(transcript_count, lab.transcript_scroll, transcript_viewport);
            lab.ensure_caret_visible();
        }
        position_text_proxy(app);
        invalidate(app);
        Ok(())
    }

    fn invalidate(app: &Rc<RefCell<Lab>>) {
        if app.borrow().render_pending {
            return;
        }
        app.borrow_mut().render_pending = true;
        let app = Rc::clone(app);
        let callback = Closure::once_into_js(move || {
            app.borrow_mut().render_pending = false;
            render(&app);
        });
        if let Some(window) = web_sys::window() {
            let _ = window.request_animation_frame(callback.unchecked_ref());
        }
    }

    fn render(app: &Rc<RefCell<Lab>>) {
        let Some(performance) = web_sys::window().and_then(|w| w.performance()) else {
            return;
        };
        let started = performance.now();
        let mut lab = app.borrow_mut();
        lab.sync_accessibility();
        let ctx = lab.context.clone();
        let layout = lab.layout();
        let accent = if lab.enabled { "#78a9ff" } else { "#8b93a7" };

        fill(&ctx, "#0b0d12");
        ctx.fill_rect(0.0, 0.0, lab.width, lab.height);
        text(
            &ctx,
            "700 23px system-ui",
            "#f4f6fb",
            24.0,
            34.0,
            "Rust/Wasm Canvas Workbench",
        );
        text(
            &ctx,
            "13px system-ui",
            "#8b93a7",
            24.0,
            58.0,
            "Ctrl/Cmd+1 Document+Data · Ctrl/Cmd+3 Diff · ? shortcuts",
        );
        text(
            &ctx,
            "12px ui-monospace, monospace",
            "#6f7789",
            (lab.width - 330.0).max(24.0),
            34.0,
            &format!(
                "A11y mirror: 9 controls  ·  DPR: {:.1}  ·  render: {:.2}ms",
                lab.dpr, lab.last_render_ms
            ),
        );

        for (rect, selected, label) in [
            (
                mode_document_data_rect(),
                lab.mode == WorkbenchMode::DocumentData,
                "1  Document + Data",
            ),
            (mode_diff_rect(), lab.mode == WorkbenchMode::Diff, "3  Diff"),
            (
                mode_transcript_rect(),
                lab.mode == WorkbenchMode::Transcript,
                "4  Transcript",
            ),
        ] {
            fill(&ctx, if selected { "#183b70" } else { "#11141b" });
            ctx.fill_rect(rect.x, rect.y, rect.width, rect.height);
            text(
                &ctx,
                "600 11px system-ui",
                if selected { "#ffffff" } else { "#8b93a7" },
                rect.x + 9.0,
                rect.y + 16.0,
                label,
            );
        }

        if lab.mode == WorkbenchMode::Diff {
            render_diff(&ctx, &lab, accent);
            render_overlay(&ctx, &lab, accent);
            lab.last_render_ms = performance.now() - started;
            return;
        }
        if lab.mode == WorkbenchMode::Transcript {
            render_transcript(&ctx, &lab, accent);
            render_overlay(&ctx, &lab, accent);
            lab.last_render_ms = performance.now() - started;
            return;
        }

        let controls = Rect {
            x: layout.action.x - 18.0,
            y: layout.action.y - if lab.width >= 760.0 { 68.0 } else { 48.0 },
            width: layout.action.width + 36.0,
            height: if lab.width >= 760.0 {
                (lab.height - 112.0).max(310.0)
            } else {
                390.0
            },
        };
        panel(&ctx, controls);
        text(
            &ctx,
            "600 14px system-ui",
            "#cdd3df",
            controls.x + 18.0,
            controls.y + 28.0,
            "CANVAS CONTROLS",
        );

        button(
            &ctx,
            layout.action,
            lab.focus == Focus::Action,
            lab.hover == Some(Hover::Action),
            accent,
            &format!("Record action  ·  {}", lab.action_count),
        );

        panel_item(
            &ctx,
            layout.toggle,
            lab.focus == Focus::Toggle,
            lab.hover == Some(Hover::Toggle),
        );
        text(
            &ctx,
            "14px system-ui",
            "#e8ebf2",
            layout.toggle.x + 14.0,
            layout.toggle.y + 25.0,
            "Custom rendering",
        );
        fill(&ctx, if lab.enabled { accent } else { "#3d4350" });
        ctx.fill_rect(
            layout.toggle.x + layout.toggle.width - 50.0,
            layout.toggle.y + 11.0,
            38.0,
            20.0,
        );
        fill(&ctx, "#ffffff");
        ctx.fill_rect(
            layout.toggle.x + layout.toggle.width - if lab.enabled { 31.0 } else { 47.0 },
            layout.toggle.y + 14.0,
            14.0,
            14.0,
        );

        text(
            &ctx,
            "12px system-ui",
            "#7f8798",
            layout.text.x,
            layout.text.y - 10.0,
            "DOCUMENT EDITOR VIA HIDDEN <TEXTAREA>",
        );
        render_editor(&ctx, &lab, layout.text, accent);

        let state_y = layout.text.y + layout.text.height + 30.0;
        text(
            &ctx,
            "600 12px ui-monospace, monospace",
            "#7f8798",
            controls.x + 18.0,
            state_y,
            "RUST STATE",
        );
        let mut state_lines = vec![
            format!("focus      {:?}", lab.focus),
            format!(
                "selection  {}..{} {}",
                lab.selection_start,
                lab.selection_end,
                if lab.selection_backward { "←" } else { "→" }
            ),
            format!("text       {} UTF-16 units", utf16_len(&lab.text)),
            format!(
                "composition {} {:?}",
                if lab.composing { "active" } else { "idle" },
                lab.composition_anchor
            ),
            format!("event      {}", lab.last_input),
        ];
        if lab.width < 760.0 {
            state_lines.truncate(2);
        }
        for (i, line) in state_lines.iter().enumerate() {
            text(
                &ctx,
                "12px ui-monospace, monospace",
                "#a9b0be",
                controls.x + 18.0,
                state_y + 24.0 + i as f64 * 20.0,
                line,
            );
        }

        panel(&ctx, layout.list);
        text(
            &ctx,
            "600 14px system-ui",
            "#cdd3df",
            layout.list.x + 18.0,
            layout.list.y + 27.0,
            "DATA WORKFLOW · VIRTUALIZED LIST",
        );
        text(
            &ctx,
            "12px ui-monospace, monospace",
            "#747d90",
            layout.list.x + layout.list.width - 230.0,
            layout.list.y + 27.0,
            &format!(
                "{} selected · {} reviewed",
                lab.selected_items.len(),
                lab.reviewed_items.len()
            ),
        );

        for (rect, hovered) in [
            (layout.data_filter, lab.hover == Some(Hover::DataFilter)),
            (
                layout.data_filter_mode,
                lab.hover == Some(Hover::DataFilterMode),
            ),
            (layout.data_sort, lab.hover == Some(Hover::DataSort)),
            (layout.data_batch, lab.hover == Some(Hover::DataBatch)),
        ] {
            panel_item(&ctx, rect, false, hovered);
        }
        let data_filter_clear = data_filter_clear_rect(layout.data_filter);
        ctx.save();
        ctx.begin_path();
        ctx.rect(
            layout.data_filter.x + 1.0,
            layout.data_filter.y + 1.0,
            (layout.data_filter.width - data_filter_clear.width - 2.0).max(0.0),
            layout.data_filter.height - 2.0,
        );
        ctx.clip();
        ctx.set_font("12px system-ui");
        if lab.overlay == Overlay::DataFilter && !lab.data_filter_modal {
            render_input_selection(
                &ctx,
                &lab,
                &lab.data_filter,
                layout.data_filter.x + 9.0,
                layout.data_filter.y + 6.0,
                17.0,
            );
        }
        text(
            &ctx,
            "12px system-ui",
            if lab.data_filter.is_empty() {
                "#747d90"
            } else {
                "#eef1f6"
            },
            layout.data_filter.x + 9.0,
            layout.data_filter.y + 19.0,
            if lab.data_filter.is_empty() {
                "Filter rows…"
            } else {
                &lab.data_filter
            },
        );
        if lab.overlay == Overlay::DataFilter && !lab.data_filter_modal {
            ctx.set_font("12px system-ui");
            let caret_units = lab
                .input
                .selection_end()
                .ok()
                .flatten()
                .unwrap_or_else(|| utf16_len(&lab.data_filter));
            let caret_byte = utf16_to_byte(&lab.data_filter, caret_units);
            let caret_x = ctx
                .measure_text(&lab.data_filter[..caret_byte])
                .map(|metrics| metrics.width())
                .unwrap_or(0.0);
            fill(&ctx, accent);
            ctx.fill_rect(
                layout.data_filter.x + 9.0 + caret_x,
                layout.data_filter.y + 6.0,
                1.5,
                17.0,
            );
            stroke(&ctx, accent, 2.0);
            ctx.stroke_rect(
                layout.data_filter.x + 1.0,
                layout.data_filter.y + 1.0,
                layout.data_filter.width - 2.0,
                layout.data_filter.height - 2.0,
            );
        }
        ctx.restore();
        panel_item(
            &ctx,
            data_filter_clear,
            false,
            lab.hover == Some(Hover::DataFilterClear),
        );
        text(
            &ctx,
            "600 12px system-ui",
            if lab.data_filter.is_empty() {
                "#667085"
            } else {
                "#c7ccd6"
            },
            data_filter_clear.x + 10.0,
            data_filter_clear.y + 19.0,
            "X",
        );
        text(
            &ctx,
            "12px system-ui",
            if lab.data_filter_error.is_some() {
                "#f38ba8"
            } else {
                "#9cc5ff"
            },
            layout.data_filter_mode.x + 9.0,
            layout.data_filter_mode.y + 19.0,
            lab.data_filter_mode.label(),
        );
        text(
            &ctx,
            "12px system-ui",
            "#c7ccd6",
            layout.data_sort.x + 9.0,
            layout.data_sort.y + 19.0,
            if lab.data_descending {
                "Sort ↓"
            } else {
                "Sort ↑"
            },
        );
        text(
            &ctx,
            "12px system-ui",
            "#c7ccd6",
            layout.data_batch.x + 9.0,
            layout.data_batch.y + 19.0,
            "Mark reviewed",
        );

        let rows = &lab.data_rows;
        if lab.data_filter_error.is_some() {
            stroke(&ctx, "#f38ba8", 1.0);
            ctx.stroke_rect(
                layout.data_filter.x + 0.5,
                layout.data_filter.y + 0.5,
                layout.data_filter.width - 1.0,
                layout.data_filter.height - 1.0,
            );
        }
        ctx.save();
        ctx.begin_path();
        ctx.rect(
            layout.list_content.x,
            layout.list_content.y,
            layout.list_content.width,
            layout.list_content.height,
        );
        ctx.clip();
        let range = visible_range_for(rows.len(), lab.scroll, layout.list_content.height);
        for position in range.clone() {
            let row = rows[position];
            let y = layout.list_content.y + position as f64 * ROW_HEIGHT - lab.scroll;
            let selected = lab.selected_items.contains(&row);
            let active = row == lab.selected;
            let hovered = lab.hover == Some(Hover::Row(row));
            if selected || hovered {
                fill(&ctx, if selected { "#183b70" } else { "#171b24" });
                ctx.fill_rect(
                    layout.list_content.x,
                    y,
                    layout.list_content.width,
                    ROW_HEIGHT - 1.0,
                );
            }
            text(
                &ctx,
                "12px ui-monospace, monospace",
                if selected { "#9cc5ff" } else { "#747d90" },
                layout.list_content.x + 14.0,
                y + 21.0,
                &format!("{:04}", row),
            );
            text(
                &ctx,
                "14px system-ui",
                if selected { "#ffffff" } else { "#c7ccd6" },
                layout.list_content.x + 70.0,
                y + 21.0,
                &format!(
                    "Canvas row {} · {}{}",
                    row + 1,
                    row_status(row),
                    if lab.reviewed_items.contains(&row) {
                        " · Reviewed"
                    } else {
                        ""
                    }
                ),
            );
            if active && lab.focus == Focus::List {
                fill(&ctx, "#78a9ff");
                ctx.fill_rect(layout.list_content.x, y, 3.0, ROW_HEIGHT - 1.0);
            }
        }
        ctx.restore();

        if !rows.is_empty() {
            let content_height = rows.len() as f64 * ROW_HEIGHT;
            let max_scroll = (content_height - layout.list_content.height).max(1.0);
            let thumb_height = (layout.list_content.height * layout.list_content.height
                / content_height)
                .clamp(24.0, layout.list_content.height);
            let thumb_y = layout.list_content.y
                + (layout.list_content.height - thumb_height) * lab.scroll / max_scroll;
            fill(&ctx, "#3c4556");
            ctx.fill_rect(
                layout.list.x + layout.list.width - 5.0,
                thumb_y,
                3.0,
                thumb_height,
            );
        }
        if lab.focus == Focus::List {
            stroke(&ctx, accent, 2.0);
            ctx.stroke_rect(
                layout.list.x + 1.0,
                layout.list.y + 1.0,
                layout.list.width - 2.0,
                layout.list.height - 2.0,
            );
        }

        render_overlay(&ctx, &lab, accent);
        lab.last_render_ms = performance.now() - started;
    }

    fn render_transcript(ctx: &CanvasRenderingContext2d, lab: &Lab, accent: &str) {
        let layout = lab.transcript_layout();
        let rows = lab.transcript_rows();
        let matches = transcript_matches(&lab.transcript_segments, &lab.transcript_query);

        panel(ctx, layout.speakers);
        text(
            ctx,
            "600 14px system-ui",
            "#cdd3df",
            layout.speakers.x + 16.0,
            layout.speakers.y + 28.0,
            "SPEAKERS",
        );
        for position in 0..=SPEAKERS.len() {
            let y = layout.speakers_content.y + position as f64 * 40.0;
            let selected = if position == 0 {
                lab.transcript_speaker_filter.is_none()
            } else {
                lab.transcript_speaker_filter == Some(position - 1)
            };
            if selected {
                fill(ctx, "#183b70");
                ctx.fill_rect(
                    layout.speakers_content.x,
                    y,
                    layout.speakers_content.width,
                    39.0,
                );
            }
            text(
                ctx,
                "13px system-ui",
                if selected { "#ffffff" } else { "#a9b0be" },
                layout.speakers_content.x + 12.0,
                y + 25.0,
                if position == 0 {
                    "All speakers"
                } else {
                    SPEAKERS[position - 1]
                },
            );
        }
        if lab.transcript_pane == TranscriptPane::Speakers {
            stroke(ctx, accent, 2.0);
            ctx.stroke_rect(
                layout.speakers.x + 1.0,
                layout.speakers.y + 1.0,
                layout.speakers.width - 2.0,
                layout.speakers.height - 2.0,
            );
        }

        panel(ctx, layout.content);
        text(
            ctx,
            "600 14px system-ui",
            "#cdd3df",
            layout.content.x + 16.0,
            layout.content.y + 27.0,
            "TRANSCRIPT",
        );
        text(
            ctx,
            "11px ui-monospace, monospace",
            "#747d90",
            layout.content.x + 16.0,
            layout.content.y + 45.0,
            &format!(
                "{} visible · {} selected · playback {}",
                rows.len(),
                lab.transcript_selected_items.len(),
                format_timestamp(lab.transcript_segments[lab.transcript_playback].seconds)
            ),
        );
        for (rect, label, hovered, active) in [
            (
                layout.search,
                if lab.transcript_query.is_empty() {
                    "/  Fuzzy search"
                } else {
                    "Fuzzy search active"
                },
                lab.hover == Some(Hover::TranscriptSearch),
                !lab.transcript_query.is_empty(),
            ),
            (
                layout.play,
                if lab.transcript_playing {
                    "Pause"
                } else {
                    "Play"
                },
                lab.hover == Some(Hover::TranscriptPlay),
                lab.transcript_playing,
            ),
            (
                layout.follow,
                "Follow",
                lab.hover == Some(Hover::TranscriptFollow),
                lab.transcript_follow,
            ),
        ] {
            panel_item(ctx, rect, active, hovered);
            text(
                ctx,
                "11px system-ui",
                if active { "#ffffff" } else { "#c7ccd6" },
                rect.x + 9.0,
                rect.y + 19.0,
                label,
            );
        }

        fill(ctx, "#0e1117");
        ctx.fill_rect(
            layout.content_body.x,
            layout.content_body.y,
            layout.content_body.width,
            layout.content_body.height,
        );
        text(
            ctx,
            "600 11px system-ui",
            "#7f8798",
            layout.content_body.x + 12.0,
            layout.content_body.y + 17.0,
            "TIME       SPEAKER     SEGMENT",
        );
        let viewport = transcript_rows_rect(layout);
        ctx.save();
        ctx.begin_path();
        ctx.rect(viewport.x, viewport.y, viewport.width, viewport.height);
        ctx.clip();
        let visible = visible_range_for(rows.len(), lab.transcript_scroll, viewport.height);
        for position in visible {
            let segment_index = rows[position];
            let segment = &lab.transcript_segments[segment_index];
            let y = viewport.y + position as f64 * ROW_HEIGHT - lab.transcript_scroll;
            let selected = lab.transcript_selected_items.contains(&segment_index);
            let playback = segment_index == lab.transcript_playback;
            let matched = matches.contains(&segment_index);
            if selected || playback || matched {
                fill(
                    ctx,
                    if playback {
                        "#3e3215"
                    } else if selected {
                        "#183b70"
                    } else {
                        "#403615"
                    },
                );
                ctx.fill_rect(viewport.x, y, viewport.width, ROW_HEIGHT - 1.0);
            }
            text(
                ctx,
                "11px ui-monospace, monospace",
                if playback { "#f6c177" } else { "#747d90" },
                viewport.x + 12.0,
                y + 21.0,
                &format_timestamp(segment.seconds),
            );
            text(
                ctx,
                "600 12px system-ui",
                if selected { "#9cc5ff" } else { "#a9b0be" },
                viewport.x + 80.0,
                y + 21.0,
                SPEAKERS[segment.speaker],
            );
            if lab.transcript_edit_target == Some(segment_index) {
                ctx.set_font("13px system-ui");
                render_input_selection(ctx, lab, &segment.text, viewport.x + 160.0, y + 7.0, 20.0);
            }
            text(
                ctx,
                "13px system-ui",
                "#eef1f6",
                viewport.x + 160.0,
                y + 21.0,
                &segment.text,
            );
            if segment_index == lab.transcript_selected
                && lab.transcript_pane == TranscriptPane::Segments
            {
                fill(ctx, accent);
                ctx.fill_rect(viewport.x, y, 3.0, ROW_HEIGHT - 1.0);
            }
        }
        ctx.restore();
        if lab.transcript_pane == TranscriptPane::Segments {
            stroke(ctx, accent, 2.0);
            ctx.stroke_rect(
                layout.content.x + 1.0,
                layout.content.y + 1.0,
                layout.content.width - 2.0,
                layout.content.height - 2.0,
            );
        }
        if !rows.is_empty() {
            let max_scroll = (rows.len() as f64 * ROW_HEIGHT - viewport.height).max(1.0);
            let thumb_height = (viewport.height * viewport.height
                / (rows.len() as f64 * ROW_HEIGHT))
                .clamp(24.0, viewport.height);
            let thumb_y =
                viewport.y + (viewport.height - thumb_height) * lab.transcript_scroll / max_scroll;
            fill(ctx, "#3c4556");
            ctx.fill_rect(
                layout.content.x + layout.content.width - 5.0,
                thumb_y,
                3.0,
                thumb_height,
            );
        }
    }

    fn render_diff(ctx: &CanvasRenderingContext2d, lab: &Lab, accent: &str) {
        let layout = lab.diff_layout();
        let rows = diff_display_rows(&lab.diff_lines, lab.diff_collapsed);
        let changes = lab
            .diff_lines
            .iter()
            .filter(|line| line.kind != DiffKind::Unchanged)
            .count();

        panel(ctx, layout.files);
        text(
            ctx,
            "600 14px system-ui",
            "#cdd3df",
            layout.files.x + 16.0,
            layout.files.y + 28.0,
            "CHANGED FILES",
        );
        for (index, name) in DIFF_FILES.iter().enumerate() {
            let y = layout.files_content.y + index as f64 * 38.0;
            let selected = index == lab.diff_file;
            if selected {
                fill(ctx, "#183b70");
                ctx.fill_rect(layout.files_content.x, y, layout.files_content.width, 37.0);
            }
            text(
                ctx,
                "13px ui-monospace, monospace",
                if selected { "#ffffff" } else { "#a9b0be" },
                layout.files_content.x + 12.0,
                y + 24.0,
                name,
            );
        }
        if lab.diff_pane == DiffPane::Files {
            stroke(ctx, accent, 2.0);
            ctx.stroke_rect(
                layout.files.x + 1.0,
                layout.files.y + 1.0,
                layout.files.width - 2.0,
                layout.files.height - 2.0,
            );
        }

        panel(ctx, layout.content);
        text(
            ctx,
            "600 14px system-ui",
            "#cdd3df",
            layout.content.x + 16.0,
            layout.content.y + 26.0,
            DIFF_FILES[lab.diff_file],
        );
        text(
            ctx,
            "11px ui-monospace, monospace",
            "#747d90",
            layout.content.x + 16.0,
            layout.content.y + 44.0,
            &format!(
                "5,000 source rows · {} displayed · {} changes",
                rows.len(),
                changes
            ),
        );
        for (rect, label, hovered) in [
            (
                layout.previous,
                "p  Previous",
                lab.hover == Some(Hover::DiffPrevious),
            ),
            (layout.next, "n  Next", lab.hover == Some(Hover::DiffNext)),
            (
                layout.collapse,
                if lab.diff_collapsed {
                    "c  Expand"
                } else {
                    "c  Collapse"
                },
                lab.hover == Some(Hover::DiffCollapse),
            ),
        ] {
            panel_item(ctx, rect, false, hovered);
            text(
                ctx,
                "11px system-ui",
                "#c7ccd6",
                rect.x + 8.0,
                rect.y + 19.0,
                label,
            );
        }

        let half = layout.content_body.width / 2.0;
        fill(ctx, "#0e1117");
        ctx.fill_rect(
            layout.content_body.x,
            layout.content_body.y,
            layout.content_body.width,
            layout.content_body.height,
        );
        text(
            ctx,
            "600 11px system-ui",
            "#7f8798",
            layout.content_body.x + 12.0,
            layout.content_body.y + 17.0,
            "OLD",
        );
        text(
            ctx,
            "600 11px system-ui",
            "#7f8798",
            layout.content_body.x + half + 12.0,
            layout.content_body.y + 17.0,
            "NEW",
        );
        let viewport = diff_rows_rect(layout);
        ctx.save();
        ctx.begin_path();
        ctx.rect(viewport.x, viewport.y, viewport.width, viewport.height);
        ctx.clip();
        let visible = visible_range_for(rows.len(), lab.diff_scroll, viewport.height);
        for position in visible {
            let y = viewport.y + position as f64 * ROW_HEIGHT - lab.diff_scroll;
            let selected =
                position >= lab.diff_selection_start && position <= lab.diff_selection_end;
            match rows[position] {
                DiffDisplayRow::Fold { count, .. } => {
                    fill(ctx, if selected { "#183b70" } else { "#171b24" });
                    ctx.fill_rect(viewport.x, y, viewport.width, ROW_HEIGHT - 1.0);
                    text(
                        ctx,
                        "12px ui-monospace, monospace",
                        "#8b93a7",
                        viewport.x + 16.0,
                        y + 21.0,
                        &format!("⋯ {} unchanged lines · click c to expand", count),
                    );
                }
                DiffDisplayRow::Line(index) => {
                    let line = &lab.diff_lines[index];
                    let (old_bg, new_bg) = match line.kind {
                        DiffKind::Unchanged => ("#0e1117", "#0e1117"),
                        DiffKind::Added => ("#0e1117", "#153a2b"),
                        DiffKind::Removed => ("#47222b", "#0e1117"),
                        DiffKind::Modified => ("#4a321b", "#35401f"),
                    };
                    fill(ctx, old_bg);
                    ctx.fill_rect(viewport.x, y, half, ROW_HEIGHT - 1.0);
                    fill(ctx, new_bg);
                    ctx.fill_rect(viewport.x + half, y, half, ROW_HEIGHT - 1.0);
                    if selected {
                        fill(ctx, "rgba(35, 91, 160, 0.45)");
                        ctx.fill_rect(viewport.x, y, viewport.width, ROW_HEIGHT - 1.0);
                    }
                    let old_number = line
                        .old_number
                        .map_or(String::new(), |value| value.to_string());
                    let new_number = line
                        .new_number
                        .map_or(String::new(), |value| value.to_string());
                    text(
                        ctx,
                        "11px ui-monospace, monospace",
                        "#747d90",
                        viewport.x + 8.0,
                        y + 21.0,
                        &old_number,
                    );
                    text(
                        ctx,
                        "12px ui-monospace, monospace",
                        if line.kind == DiffKind::Removed {
                            "#ffc0c8"
                        } else {
                            "#c7ccd6"
                        },
                        viewport.x + 50.0,
                        y + 21.0,
                        &line.old_text,
                    );
                    text(
                        ctx,
                        "11px ui-monospace, monospace",
                        "#747d90",
                        viewport.x + half + 8.0,
                        y + 21.0,
                        &new_number,
                    );
                    text(
                        ctx,
                        "12px ui-monospace, monospace",
                        if line.kind == DiffKind::Added {
                            "#b8f2c8"
                        } else {
                            "#c7ccd6"
                        },
                        viewport.x + half + 50.0,
                        y + 21.0,
                        &line.new_text,
                    );
                    if position == lab.diff_selected && lab.diff_pane == DiffPane::Content {
                        fill(ctx, accent);
                        ctx.fill_rect(viewport.x, y, 3.0, ROW_HEIGHT - 1.0);
                    }
                }
            }
        }
        ctx.restore();
        stroke(ctx, "#303746", 1.0);
        ctx.begin_path();
        ctx.move_to(layout.content_body.x + half, layout.content_body.y);
        ctx.line_to(
            layout.content_body.x + half,
            layout.content_body.y + layout.content_body.height,
        );
        ctx.stroke();

        if !rows.is_empty() {
            let max_scroll = (rows.len() as f64 * ROW_HEIGHT - viewport.height).max(1.0);
            let thumb_height = (viewport.height * viewport.height
                / (rows.len() as f64 * ROW_HEIGHT))
                .clamp(24.0, viewport.height);
            let thumb_y =
                viewport.y + (viewport.height - thumb_height) * lab.diff_scroll / max_scroll;
            fill(ctx, "#3c4556");
            ctx.fill_rect(
                layout.content.x + layout.content.width - 5.0,
                thumb_y,
                3.0,
                thumb_height,
            );
        }
        if lab.diff_pane == DiffPane::Content {
            stroke(ctx, accent, 2.0);
            ctx.stroke_rect(
                layout.content.x + 1.0,
                layout.content.y + 1.0,
                layout.content.width - 2.0,
                layout.content.height - 2.0,
            );
        }
    }

    fn render_input_selection(
        ctx: &CanvasRenderingContext2d,
        lab: &Lab,
        value: &str,
        x: f64,
        top: f64,
        height: f64,
    ) {
        let start = lab.input.selection_start().ok().flatten().unwrap_or(0);
        let end = lab.input.selection_end().ok().flatten().unwrap_or(start);
        let start_byte = utf16_to_byte(value, start);
        let start_x = ctx
            .measure_text(&value[..start_byte])
            .map(|metrics| metrics.width())
            .unwrap_or(0.0);
        if start == end {
            fill(ctx, "#78a9ff");
            ctx.fill_rect(x + start_x, top, 1.5, height);
            return;
        }
        let end_byte = utf16_to_byte(value, end);
        let end_x = ctx
            .measure_text(&value[..end_byte])
            .map(|metrics| metrics.width())
            .unwrap_or(start_x);
        fill(ctx, "#274d7e");
        ctx.fill_rect(x + start_x, top, (end_x - start_x).max(2.0), height);
    }

    fn render_overlay(ctx: &CanvasRenderingContext2d, lab: &Lab, accent: &str) {
        if lab.overlay == Overlay::None
            || (lab.overlay == Overlay::DataFilter && !lab.data_filter_modal)
        {
            return;
        }
        let overlay = overlay_rect(lab);
        fill(ctx, "rgba(5, 7, 11, 0.75)");
        ctx.fill_rect(0.0, 0.0, lab.width, lab.height);
        panel(ctx, overlay);
        stroke(ctx, accent, 2.0);
        ctx.stroke_rect(
            overlay.x + 1.0,
            overlay.y + 1.0,
            overlay.width - 2.0,
            overlay.height - 2.0,
        );

        if lab.overlay == Overlay::Shortcuts {
            text(
                ctx,
                "600 14px system-ui",
                "#eef1f6",
                overlay.x + 20.0,
                overlay.y + 30.0,
                "KEYBOARD SHORTCUTS",
            );
            text(
                ctx,
                "12px system-ui",
                "#7f8798",
                overlay.x + overlay.width - 105.0,
                overlay.y + 30.0,
                "Esc to close",
            );
            for (row, (key, action)) in SHORTCUTS.iter().enumerate() {
                let y = overlay.y + 68.0 + row as f64 * 32.0;
                text(
                    ctx,
                    "600 13px ui-monospace, monospace",
                    "#9cc5ff",
                    overlay.x + 20.0,
                    y,
                    key,
                );
                text(
                    ctx,
                    "13px system-ui",
                    "#c7ccd6",
                    overlay.x + 180.0,
                    y,
                    action,
                );
            }
            return;
        }

        let (label, value) = match lab.overlay {
            Overlay::Search => ("SEARCH DOCUMENT", lab.search_query.as_str()),
            Overlay::Command => ("COMMAND PALETTE", lab.command_query.as_str()),
            Overlay::DataFilter => ("FILTER DATA ROWS", lab.data_filter.as_str()),
            Overlay::TranscriptSearch => ("FUZZY SEARCH TRANSCRIPT", lab.transcript_query.as_str()),
            Overlay::Shortcuts | Overlay::None => unreachable!(),
        };
        text(
            ctx,
            "600 11px system-ui",
            "#7f8798",
            overlay.x + 16.0,
            overlay.y + 18.0,
            label,
        );
        let shown = if value.is_empty() {
            "Type to filter…"
        } else {
            value
        };
        if !value.is_empty() {
            ctx.set_font(EDITOR_FONT);
            render_input_selection(ctx, lab, value, overlay.x + 16.0, overlay.y + 27.0, 21.0);
        }
        text(
            ctx,
            EDITOR_FONT,
            if value.is_empty() {
                "#667085"
            } else {
                "#eef1f6"
            },
            overlay.x + 16.0,
            overlay.y + 43.0,
            shown,
        );
        if value.is_empty() {
            fill(ctx, accent);
            ctx.fill_rect(overlay.x + 16.0, overlay.y + 27.0, 1.5, 21.0);
        } else {
            let caret_units = lab
                .input
                .selection_end()
                .ok()
                .flatten()
                .unwrap_or_else(|| utf16_len(value));
            let caret_byte = utf16_to_byte(value, caret_units);
            let caret_x = ctx
                .measure_text(&value[..caret_byte])
                .map(|metrics| metrics.width())
                .unwrap_or(0.0);
            fill(ctx, accent);
            ctx.fill_rect(overlay.x + 16.0 + caret_x, overlay.y + 27.0, 1.5, 21.0);
        }

        match lab.overlay {
            Overlay::Search => {
                let count = find_utf16_matches(&lab.text, &lab.search_query).len();
                text(
                    ctx,
                    "12px system-ui",
                    "#a9b0be",
                    overlay.x + overlay.width - 110.0,
                    overlay.y + 42.0,
                    &format!("{} matches", count),
                );
            }
            Overlay::TranscriptSearch => {
                let count =
                    transcript_matches(&lab.transcript_segments, &lab.transcript_query).len();
                text(
                    ctx,
                    "12px system-ui",
                    "#a9b0be",
                    overlay.x + overlay.width - 110.0,
                    overlay.y + 42.0,
                    &format!("{} matches", count),
                );
            }
            Overlay::DataFilter => {
                let status = if lab.data_filter_pending {
                    "Regex pending…".into()
                } else {
                    lab.data_filter_error.as_ref().map_or_else(
                        || {
                            format!(
                                "{} · {} rows · {:.2}ms",
                                lab.data_filter_mode.label(),
                                lab.data_rows.len(),
                                lab.data_filter_ms
                            )
                        },
                        |_| "Invalid regex".into(),
                    )
                };
                text(
                    ctx,
                    "12px system-ui",
                    if lab.data_filter_error.is_some() {
                        "#f38ba8"
                    } else {
                        "#a9b0be"
                    },
                    overlay.x + overlay.width - 170.0,
                    overlay.y + 42.0,
                    &status,
                );
            }
            Overlay::Command => {
                let commands = matching_commands(&lab.command_query);
                for (row, command) in commands.iter().take(4).enumerate() {
                    let y = overlay.y + 60.0 + row as f64 * 38.0;
                    if row == lab.command_selected {
                        fill(ctx, "#183b70");
                        ctx.fill_rect(overlay.x + 8.0, y, overlay.width - 16.0, 34.0);
                    }
                    text(
                        ctx,
                        "14px system-ui",
                        if row == lab.command_selected {
                            "#ffffff"
                        } else {
                            "#c7ccd6"
                        },
                        overlay.x + 18.0,
                        y + 22.0,
                        COMMANDS[*command],
                    );
                }
            }
            Overlay::Shortcuts | Overlay::None => {}
        }
    }

    fn render_editor(ctx: &CanvasRenderingContext2d, lab: &Lab, editor: Rect, accent: &str) {
        panel_item(
            ctx,
            editor,
            lab.focus == Focus::Text,
            lab.hover == Some(Hover::Text),
        );
        ctx.save();
        ctx.begin_path();
        ctx.rect(
            editor.x + 1.0,
            editor.y + 1.0,
            editor.width - 2.0,
            editor.height - 2.0,
        );
        ctx.clip();
        ctx.set_font(EDITOR_FONT);

        let selection_start = lab.selection_start.min(lab.selection_end);
        let selection_end = lab.selection_start.max(lab.selection_end);
        let composition = lab
            .composition_anchor
            .map(|anchor| (anchor.min(lab.selection_end), anchor.max(lab.selection_end)));
        let search_matches = find_utf16_matches(&lab.text, &lab.search_query);
        let mut line_start = 0;
        let lines: Vec<&str> = lab.text.split('\n').collect();
        for (index, line) in lines.iter().enumerate() {
            let line_units = utf16_len(line);
            let line_end = line_start + line_units;
            let top =
                editor.y + EDITOR_PADDING + index as f64 * EDITOR_LINE_HEIGHT - lab.editor_scroll;
            let baseline = top + 17.0;
            if top + EDITOR_LINE_HEIGHT < editor.y || top > editor.y + editor.height {
                line_start = line_end + 1;
                continue;
            }

            for &(match_start, match_end) in &search_matches {
                if match_end <= line_start || match_start > line_end {
                    continue;
                }
                let local_start = match_start.saturating_sub(line_start).min(line_units);
                let local_end = match_end.saturating_sub(line_start).min(line_units);
                let start_byte = utf16_to_byte(line, local_start);
                let end_byte = utf16_to_byte(line, local_end);
                let start_x = ctx
                    .measure_text(&line[..start_byte])
                    .map(|metrics| metrics.width())
                    .unwrap_or(0.0);
                let end_x = ctx
                    .measure_text(&line[..end_byte])
                    .map(|metrics| metrics.width())
                    .unwrap_or(start_x);
                fill(ctx, "#6a531d");
                ctx.fill_rect(
                    editor.x + EDITOR_PADDING + start_x,
                    top,
                    (end_x - start_x).max(2.0),
                    EDITOR_LINE_HEIGHT,
                );
            }

            if selection_start != selection_end
                && selection_end > line_start
                && selection_start <= line_end
            {
                let local_start = selection_start.saturating_sub(line_start).min(line_units);
                let local_end = selection_end.saturating_sub(line_start).min(line_units);
                let start_byte = utf16_to_byte(line, local_start);
                let end_byte = utf16_to_byte(line, local_end);
                let start_x = ctx
                    .measure_text(&line[..start_byte])
                    .map(|metrics| metrics.width())
                    .unwrap_or(0.0);
                let mut end_x = ctx
                    .measure_text(&line[..end_byte])
                    .map(|metrics| metrics.width())
                    .unwrap_or(start_x);
                if selection_end > line_end && index + 1 < lines.len() {
                    end_x += 8.0;
                }
                fill(ctx, "#274d7e");
                ctx.fill_rect(
                    editor.x + EDITOR_PADDING + start_x,
                    top,
                    (end_x - start_x).max(2.0),
                    EDITOR_LINE_HEIGHT,
                );
            }

            text(
                ctx,
                EDITOR_FONT,
                if line.is_empty() {
                    "#667085"
                } else {
                    "#eef1f6"
                },
                editor.x + EDITOR_PADDING,
                baseline,
                if line.is_empty() { " " } else { line },
            );

            if let Some((composition_start, composition_end)) = composition
                && lab.composing
                && composition_end > line_start
                && composition_start <= line_end
            {
                let local_start = composition_start.saturating_sub(line_start).min(line_units);
                let local_end = composition_end.saturating_sub(line_start).min(line_units);
                let start_byte = utf16_to_byte(line, local_start);
                let end_byte = utf16_to_byte(line, local_end);
                let start_x = ctx
                    .measure_text(&line[..start_byte])
                    .map(|metrics| metrics.width())
                    .unwrap_or(0.0);
                let end_x = ctx
                    .measure_text(&line[..end_byte])
                    .map(|metrics| metrics.width())
                    .unwrap_or(start_x);
                stroke(ctx, "#f6c177", 2.0);
                ctx.begin_path();
                ctx.move_to(editor.x + EDITOR_PADDING + start_x, baseline + 3.0);
                ctx.line_to(
                    editor.x + EDITOR_PADDING + end_x.max(start_x + 3.0),
                    baseline + 3.0,
                );
                ctx.stroke();
            }
            line_start = line_end + 1;
        }

        if lab.focus == Focus::Text && selection_start == selection_end {
            let (x, y) = caret_point(ctx, &lab.text, lab.selection_end, editor, lab.editor_scroll);
            fill(ctx, accent);
            ctx.fill_rect(x, y + 2.0, 1.5, EDITOR_LINE_HEIGHT - 4.0);
        }
        ctx.restore();
    }

    fn fill(ctx: &CanvasRenderingContext2d, color: &str) {
        ctx.set_fill_style_str(color);
    }

    fn stroke(ctx: &CanvasRenderingContext2d, color: &str, width: f64) {
        ctx.set_stroke_style_str(color);
        ctx.set_line_width(width);
    }

    fn text(ctx: &CanvasRenderingContext2d, font: &str, color: &str, x: f64, y: f64, value: &str) {
        ctx.set_font(font);
        ctx.set_text_baseline("alphabetic");
        fill(ctx, color);
        let _ = ctx.fill_text(value, x, y);
    }

    fn panel(ctx: &CanvasRenderingContext2d, rect: Rect) {
        fill(ctx, "#11141b");
        ctx.fill_rect(rect.x, rect.y, rect.width, rect.height);
        stroke(ctx, "#262c38", 1.0);
        ctx.stroke_rect(
            rect.x + 0.5,
            rect.y + 0.5,
            rect.width - 1.0,
            rect.height - 1.0,
        );
    }

    fn panel_item(ctx: &CanvasRenderingContext2d, rect: Rect, focused: bool, hovered: bool) {
        fill(ctx, if hovered { "#202632" } else { "#171b24" });
        ctx.fill_rect(rect.x, rect.y, rect.width, rect.height);
        stroke(
            ctx,
            if focused { "#78a9ff" } else { "#303746" },
            if focused { 2.0 } else { 1.0 },
        );
        ctx.stroke_rect(
            rect.x + 0.5,
            rect.y + 0.5,
            rect.width - 1.0,
            rect.height - 1.0,
        );
    }

    fn button(
        ctx: &CanvasRenderingContext2d,
        rect: Rect,
        focused: bool,
        hovered: bool,
        accent: &str,
        label: &str,
    ) {
        fill(ctx, if hovered { "#2f6fcb" } else { accent });
        ctx.fill_rect(rect.x, rect.y, rect.width, rect.height);
        if focused {
            stroke(ctx, "#dbeaff", 2.0);
            ctx.stroke_rect(
                rect.x + 2.0,
                rect.y + 2.0,
                rect.width - 4.0,
                rect.height - 4.0,
            );
        }
        text(
            ctx,
            "600 14px system-ui",
            "#07101f",
            rect.x + 14.0,
            rect.y + 27.0,
            label,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_testing_excludes_right_and_bottom_edges() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 30.0,
            height: 40.0,
        };
        assert!(rect.contains(10.0, 20.0));
        assert!(rect.contains(39.9, 59.9));
        assert!(!rect.contains(40.0, 60.0));
    }

    #[test]
    fn list_ranges_work_in_both_directions() {
        assert_eq!(item_range(3, 6), 3..=6);
        assert_eq!(item_range(6, 3), 3..=6);
    }

    #[test]
    fn generated_diff_contains_changes_and_collapses_unchanged_runs() {
        let lines = generate_diff(0, 500);
        assert!(lines.iter().any(|line| line.kind == DiffKind::Added));
        assert!(lines.iter().any(|line| line.kind == DiffKind::Removed));
        assert!(lines.iter().any(|line| line.kind == DiffKind::Modified));
        let expanded = diff_display_rows(&lines, false);
        let collapsed = diff_display_rows(&lines, true);
        assert_eq!(expanded.len(), 500);
        assert!(collapsed.len() < expanded.len());
        assert!(
            collapsed
                .iter()
                .any(|row| matches!(row, DiffDisplayRow::Fold { .. }))
        );
        assert_ne!(WorkbenchMode::DocumentData, WorkbenchMode::Diff);
        assert_ne!(DiffPane::Files, DiffPane::Content);
    }

    #[test]
    fn generated_transcript_has_timestamps_speakers_and_search() {
        let segments = generate_transcript(2_000);
        assert_eq!(segments.len(), 2_000);
        assert_eq!(segments[0].speaker, 0);
        assert_eq!(segments[4].speaker, 0);
        assert_eq!(format_timestamp(0), "00:00");
        assert_eq!(format_timestamp(3_661), "1:01:01");
        assert!(!transcript_matches(&segments, "plybcktmstmp").is_empty());
        assert_eq!(transcript_matches(&segments, "sgmnt1999"), vec![1_998]);
        assert!(transcript_matches(&segments, "timestamp playback").is_empty());
        assert_eq!(transcript_matches(&segments, ""), Vec::<usize>::new());
        assert_ne!(WorkbenchMode::Diff, WorkbenchMode::Transcript);
        assert_ne!(TranscriptPane::Speakers, TranscriptPane::Segments);
    }

    #[test]
    fn directional_focus_follows_the_ui_geometry() {
        assert_eq!(directional_focus(Focus::List, "h"), Focus::Text);
        assert_eq!(directional_focus(Focus::Text, "l"), Focus::List);
        assert_eq!(directional_focus(Focus::Action, "j"), Focus::Toggle);
        assert_eq!(directional_focus(Focus::Text, "k"), Focus::Toggle);
    }

    #[test]
    fn filter_modes_cycle_and_have_labels() {
        assert_eq!(FilterMode::Fuzzy.label(), "Fuzzy");
        assert_eq!(FilterMode::Fuzzy.next(), FilterMode::Regex);
        assert_eq!(FilterMode::Regex.next(), FilterMode::Literal);
        assert_eq!(FilterMode::Literal.next(), FilterMode::Fuzzy);
    }

    #[test]
    fn data_filter_and_sort_preserve_row_ids() {
        let index = data_search_index();
        let rows = filtered_data_rows(&index, "blocked", FilterMode::Literal, false).unwrap();
        assert_eq!(rows[..3], [2, 5, 8]);
        let rows = filtered_data_rows(&index, "row 42", FilterMode::Literal, false).unwrap();
        assert_eq!(rows.first(), Some(&41));
        assert!(rows.contains(&419));
        let rows = filtered_data_rows(&index, "blocked", FilterMode::Literal, true).unwrap();
        assert_eq!(rows.first(), Some(&9998));
    }

    #[test]
    fn regex_filter_reports_errors_and_matches_patterns() {
        let index = data_search_index();
        let rows = filtered_data_rows(
            &index,
            r"canvas row (3|6) (ready|review|blocked)$",
            FilterMode::Regex,
            false,
        )
        .unwrap();
        assert_eq!(rows, vec![2, 5]);
        assert!(filtered_data_rows(&index, "[", FilterMode::Regex, false).is_err());
    }

    #[test]
    fn fuzzy_filter_preserves_the_selected_sort_order() {
        let index = vec![
            "canvas row 234 blocked".into(),
            "canvas row 9234 blocked".into(),
            "ready".into(),
        ];
        assert_eq!(
            filtered_data_rows(&index, "canva234blo", FilterMode::Fuzzy, false).unwrap(),
            vec![0, 1]
        );
        assert_eq!(
            filtered_data_rows(&index, "canva234blo", FilterMode::Fuzzy, true).unwrap(),
            vec![1, 0]
        );
    }

    #[test]
    fn virtualization_returns_only_visible_rows() {
        assert_eq!(visible_range_for(ITEM_COUNT, 0.0, 68.0), 0..3);
        assert_eq!(visible_range_for(ITEM_COUNT, 340.0, 68.0), 10..13);
        assert_eq!(
            visible_range_for(ITEM_COUNT, ITEM_COUNT as f64 * ROW_HEIGHT, 68.0),
            ITEM_COUNT..ITEM_COUNT
        );
    }

    #[test]
    fn scrolling_is_clamped_to_content() {
        let viewport = 340.0;
        assert_eq!(clamp_scroll_for(ITEM_COUNT, -20.0, viewport), 0.0);
        assert_eq!(
            clamp_scroll_for(ITEM_COUNT, f64::MAX, viewport),
            ITEM_COUNT as f64 * ROW_HEIGHT - viewport
        );
    }

    #[test]
    fn utf16_offsets_map_to_rust_string_boundaries() {
        let text = "a🦀é\n日";
        assert_eq!(utf16_len(text), 6);
        assert_eq!(utf16_to_byte(text, 0), 0);
        assert_eq!(&text[..utf16_to_byte(text, 1)], "a");
        assert_eq!(&text[..utf16_to_byte(text, 3)], "a🦀");
        assert_eq!(&text[..utf16_to_byte(text, 6)], text);
    }

    #[test]
    fn utf16_offsets_identify_multiline_caret_rows() {
        let text = "one\n🦀 two\nthree";
        assert_eq!(line_for_utf16(text, 0), 0);
        assert_eq!(line_for_utf16(text, 4), 1);
        assert_eq!(line_for_utf16(text, 11), 2);
    }

    #[test]
    fn trailing_newline_counts_as_an_editable_row() {
        assert_eq!(text_line_count("one\n"), 2);
        assert_eq!(line_for_utf16("one\n", 4), 1);
    }

    #[test]
    fn horizontal_navigation_moves_by_grapheme_not_scalar() {
        let text = "a🇧🇷e\u{301}🦀";
        let first = next_grapheme_utf16(text, 0);
        let second = next_grapheme_utf16(text, first);
        let third = next_grapheme_utf16(text, second);
        assert_eq!(&text[..utf16_to_byte(text, first)], "a");
        assert_eq!(&text[..utf16_to_byte(text, second)], "a🇧🇷");
        assert_eq!(&text[..utf16_to_byte(text, third)], "a🇧🇷e\u{301}");
        assert_eq!(previous_grapheme_utf16(text, third), second);
    }

    #[test]
    fn home_and_end_use_the_current_utf16_line() {
        let text = "one\n🦀 two\nthree";
        assert_eq!(line_bounds_for_utf16(text, 7), (4, 10));
        assert_eq!(line_bounds_for_utf16(text, 10), (4, 10));
    }

    #[test]
    fn search_matches_use_browser_utf16_offsets() {
        assert_eq!(find_utf16_matches("🦀 one 🦀", "🦀"), vec![(0, 2), (7, 9)]);
        assert!(find_utf16_matches("text", "").is_empty());
    }
}
