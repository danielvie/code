//! Single-line label editing. Caret and selection are painted independently of glyph layout.
use gpui::{
    App, Bounds, ClipboardItem, ContentMask, CursorStyle, Keystroke, Pixels, Point, ShapedLine,
    TextRun, canvas, div, fill, point, prelude::*, px, rgb, size,
};
use std::{cell::RefCell, ops::Range, rc::Rc, time::Instant};
use unicode_segmentation::UnicodeSegmentation;

pub struct TextBuffer {
    pub text: String,
    pub cursor: usize,
    anchor: usize,
}
impl TextBuffer {
    pub fn new(text: String) -> Self {
        Self {
            cursor: text.len(),
            anchor: 0,
            text,
        }
    }
    pub fn selection(&self) -> Range<usize> {
        self.anchor.min(self.cursor)..self.anchor.max(self.cursor)
    }
    fn select_all(&mut self) {
        self.anchor = 0;
        self.cursor = self.text.len();
    }
    fn previous(&self) -> usize {
        self.text
            .grapheme_indices(true)
            .rev()
            .find(|(i, _)| *i < self.cursor)
            .map_or(0, |(i, _)| i)
    }
    fn next(&self) -> usize {
        self.text
            .grapheme_indices(true)
            .find(|(i, _)| *i > self.cursor)
            .map_or(self.text.len(), |(i, _)| i)
    }
    fn word_left(&self) -> usize {
        self.text
            .unicode_word_indices()
            .rev()
            .find(|(i, _)| *i < self.cursor)
            .map_or(0, |(i, _)| i)
    }
    fn word_right(&self) -> usize {
        self.text
            .unicode_word_indices()
            .find(|(i, _)| *i > self.cursor)
            .map_or(self.text.len(), |(i, _)| i)
    }
    fn move_to(&mut self, target: usize, extend: bool) {
        self.cursor = target;
        if !extend {
            self.anchor = target;
        }
    }
    fn move_horizontal(&mut self, right: bool, word: bool, extend: bool) {
        let range = self.selection();
        let target = if !extend && !word && !range.is_empty() {
            if right { range.end } else { range.start }
        } else if word {
            if right {
                self.word_right()
            } else {
                self.word_left()
            }
        } else if right {
            self.next()
        } else {
            self.previous()
        };
        self.move_to(target, extend);
    }
    fn replace(&mut self, value: &str) {
        let clean: String = value.chars().filter(|c| !c.is_control()).collect();
        let range = self.selection();
        let remaining =
            self.text[..range.start].chars().count() + self.text[range.end..].chars().count();
        if remaining + clean.chars().count() > 64 {
            return;
        }
        self.text.replace_range(range.clone(), &clean);
        self.move_to(range.start + clean.len(), false);
    }
    fn delete(&mut self, backward: bool, word: bool) {
        if self.selection().is_empty() {
            self.move_horizontal(!backward, word, true);
        }
        self.replace("");
    }
    fn select_word(&mut self, index: usize) {
        if let Some((start, part)) = self
            .text
            .split_word_bound_indices()
            .find(|(start, part)| index >= *start && index < start + part.len())
        {
            self.anchor = start;
            self.cursor = start + part.len();
        } else {
            self.move_to(index, false);
        }
    }
}

struct Layout {
    line: ShapedLine,
    bounds: Bounds<Pixels>,
    scroll: Pixels,
}

pub struct LabelEditor {
    pub buffer: TextBuffer,
    pub selecting: bool,
    layout: Rc<RefCell<Option<Layout>>>,
    last_input: Instant,
    caret_visible: bool,
}
impl LabelEditor {
    pub fn new(text: String) -> Self {
        Self {
            buffer: TextBuffer::new(text),
            selecting: false,
            layout: Rc::new(RefCell::new(None)),
            last_input: Instant::now(),
            caret_visible: true,
        }
    }
    fn reset_blink(&mut self) {
        self.last_input = Instant::now();
        self.caret_visible = true;
    }
    pub fn tick(&mut self) -> bool {
        let visible = self.buffer.selection().is_empty()
            && (self.last_input.elapsed().as_millis() / 500).is_multiple_of(2);
        let changed = self.caret_visible != visible;
        self.caret_visible = visible;
        changed
    }
    pub fn key(&mut self, k: &Keystroke, cx: &mut App) {
        let control = k.modifiers.control || k.modifiers.platform;
        match k.key.as_str() {
            "left" => self
                .buffer
                .move_horizontal(false, control, k.modifiers.shift),
            "right" => self
                .buffer
                .move_horizontal(true, control, k.modifiers.shift),
            "home" => self.buffer.move_to(0, k.modifiers.shift),
            "end" => self
                .buffer
                .move_to(self.buffer.text.len(), k.modifiers.shift),
            "backspace" => self.buffer.delete(true, control),
            "delete" => self.buffer.delete(false, control),
            "a" if control => self.buffer.select_all(),
            "c" | "x" if control => {
                let range = self.buffer.selection();
                if !range.is_empty() {
                    cx.write_to_clipboard(ClipboardItem::new_string(
                        self.buffer.text[range].to_owned(),
                    ));
                    if k.key == "x" {
                        self.buffer.replace("");
                    }
                }
            }
            "v" if control => {
                if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                    self.buffer.replace(&text);
                }
            }
            _ if !control && !k.modifiers.alt => {
                if k.key == "space" {
                    self.buffer.replace(" ");
                } else if let Some(text) = &k.key_char {
                    self.buffer.replace(text);
                }
            }
            _ => {}
        }
        self.reset_blink();
    }
    fn index_at(&self, position: Point<Pixels>) -> Option<usize> {
        let cached = self.layout.borrow();
        let layout = cached.as_ref()?;
        // Ignore an old shaped line if a key arrived before the next paint.
        if layout.line.text.as_ref() != self.buffer.text {
            return None;
        }
        let index = layout
            .line
            .closest_index_for_x(position.x - layout.bounds.left() + layout.scroll);
        Some(
            self.buffer
                .text
                .grapheme_indices(true)
                .map(|(i, _)| i)
                .chain(std::iter::once(self.buffer.text.len()))
                .min_by_key(|i| i.abs_diff(index))
                .unwrap_or(0),
        )
    }
    pub fn mouse_down(&mut self, position: Point<Pixels>, shift: bool, clicks: usize) {
        if let Some(index) = self.index_at(position) {
            if clicks >= 3 {
                self.buffer.select_all();
            } else if clicks == 2 && !shift {
                self.buffer.select_word(index);
            } else {
                self.buffer.move_to(index, shift);
            }
            self.selecting = clicks == 1;
            self.reset_blink();
        }
    }
    pub fn mouse_move(&mut self, position: Point<Pixels>) {
        if let Some(index) = self.index_at(position) {
            self.buffer.move_to(index, true);
            self.reset_blink();
        }
    }
    pub fn element(&self, focused: bool) -> impl IntoElement {
        let text = self.buffer.text.clone();
        let range = self.buffer.selection();
        let cursor = self.buffer.cursor;
        let caret_visible = self.caret_visible && range.is_empty() && focused;
        let cache = self.layout.clone();
        div()
            .w_full()
            .h(px(32.))
            .px_2()
            .py_1()
            .bg(rgb(0xf7f9fc))
            .border_1()
            .border_color(rgb(0x9bb8f3))
            .cursor(CursorStyle::IBeam)
            .overflow_hidden()
            .child(
                canvas(
                    move |bounds, window, _| {
                        let style = window.text_style();
                        // Shape the full string without inserting a caret or splitting at selection edges.
                        let run = TextRun {
                            len: text.len(),
                            font: style.font(),
                            color: rgb(0x263247).into(),
                            background_color: None,
                            underline: None,
                            strikethrough: None,
                        };
                        let line =
                            window
                                .text_system()
                                .shape_line(text.into(), px(14.), &[run], None);
                        let previous = cache.borrow().as_ref().map_or(px(0.), |l| l.scroll);
                        let caret_x = line.x_for_index(cursor);
                        let width = (bounds.size.width - px(3.)).max(px(1.));
                        let scroll = previous
                            .max(caret_x - width)
                            .min(caret_x)
                            .max(px(0.))
                            .min((line.width - width).max(px(0.)));
                        let origin = point(bounds.left() - scroll, bounds.top());
                        *cache.borrow_mut() = Some(Layout {
                            line: line.clone(),
                            bounds,
                            scroll,
                        });
                        (line, origin)
                    },
                    move |bounds, (line, origin), window, cx| {
                        window.with_content_mask(Some(ContentMask { bounds }), |window| {
                            let height = bounds.size.height;
                            if !range.is_empty() {
                                let selected = Bounds::new(
                                    point(origin.x + line.x_for_index(range.start), bounds.top()),
                                    size(
                                        line.x_for_index(range.end) - line.x_for_index(range.start),
                                        height,
                                    ),
                                );
                                window.paint_quad(fill(
                                    selected,
                                    rgb(if focused { 0x2563eb } else { 0xa7bfe9 }),
                                ));
                                let _ = line.paint(origin, height, window, cx);
                                // Repaint only selected glyphs in white, retaining exactly the same layout.
                                // The full font/string pair preserves the original glyph metrics.
                                let style = window.text_style();
                                let white = TextRun {
                                    len: line.len(),
                                    font: style.font(),
                                    color: rgb(0xffffff).into(),
                                    background_color: None,
                                    underline: None,
                                    strikethrough: None,
                                };
                                let highlighted = window.text_system().shape_line(
                                    line.text.clone(),
                                    px(14.),
                                    &[white],
                                    None,
                                );
                                window.with_content_mask(
                                    Some(ContentMask { bounds: selected }),
                                    |window| {
                                        let _ = highlighted.paint(origin, height, window, cx);
                                    },
                                );
                            } else {
                                let _ = line.paint(origin, height, window, cx);
                            }
                            if caret_visible {
                                window.paint_quad(fill(
                                    Bounds::new(
                                        point(
                                            origin.x + line.x_for_index(cursor),
                                            bounds.top() + px(2.),
                                        ),
                                        size(px(1.5), height - px(4.)),
                                    ),
                                    rgb(0x164bca),
                                ));
                            }
                        });
                    },
                )
                .size_full(),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arrows_collapse_selection_without_editing_text() {
        let mut b = TextBuffer::new("Control system".into());
        b.move_horizontal(false, false, false);
        assert_eq!(b.cursor, 0);
        b.move_horizontal(true, false, false);
        assert_eq!(b.cursor, 1);
        assert_eq!(b.text, "Control system");
    }
    #[test]
    fn shift_selection_extends_and_reverses_around_fixed_anchor() {
        let mut b = TextBuffer::new("abcdef".into());
        b.move_to(3, false);
        b.move_horizontal(false, false, true);
        b.move_horizontal(false, false, true);
        assert_eq!(b.selection(), 1..3);
        b.move_to(5, true);
        assert_eq!(b.selection(), 3..5);
        b.replace("X");
        assert_eq!(b.text, "abcXf");
        assert_eq!(b.selection(), 4..4);
    }
    #[test]
    fn control_arrows_and_control_shift_move_by_words() {
        let mut b = TextBuffer::new("Control system / input".into());
        b.move_to(0, false);
        b.move_horizontal(true, true, false);
        assert_eq!(b.cursor, 8);
        b.move_horizontal(true, true, true);
        assert_eq!(b.selection(), 8..17);
        b.move_horizontal(false, true, false);
        assert_eq!(b.cursor, 8);
        assert!(b.selection().is_empty());
    }
    #[test]
    fn graphemes_are_not_split_by_arrows_or_deletion() {
        let mut b = TextBuffer::new("a👩‍💻e\u{301}".into());
        b.move_to(b.text.len(), false);
        b.delete(true, false);
        assert_eq!(b.text, "a👩‍💻");
        b.move_horizontal(false, false, true);
        assert_eq!(&b.text[b.selection()], "👩‍💻");
        b.replace("b");
        assert_eq!(b.text, "ab");
    }
    #[test]
    fn replacement_limit_uses_remaining_text_and_filters_newlines() {
        let mut b = TextBuffer::new("a".repeat(64));
        b.replace("new\nlabel");
        assert_eq!(b.text, "newlabel");
        b.replace(&"b".repeat(64));
        assert_eq!(b.text, "newlabel");
    }
    #[test]
    fn word_deletion_and_backwards_selection_replace_correctly() {
        let mut b = TextBuffer::new("Control system".into());
        b.move_to(b.text.len(), false);
        b.delete(true, true);
        assert_eq!(b.text, "Control ");
        b.move_to(0, true);
        b.replace("Sensor");
        assert_eq!(b.text, "Sensor");
    }
    #[test]
    fn blinking_resets_after_input() {
        let mut e = LabelEditor::new("abc".into());
        e.buffer.move_to(1, false);
        e.last_input = Instant::now() - std::time::Duration::from_millis(600);
        assert!(e.tick());
        assert!(!e.caret_visible);
        e.reset_blink();
        assert!(e.caret_visible);
    }
}
