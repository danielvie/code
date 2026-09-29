//! Prototype 1: can a GPUI canvas support direct block/port manipulation and live routing?
//! Diagram changes autosave locally.
use crate::diagram_model::{Block, COLORS, Diagram, GRID, Port, Pos, WIDTH};
use crate::diagram_router::{Junction, Route, junctions, route_all, wire_distance};
use crate::diagram_selection::{GroupDrag, Marquee, clicked_blocks};
use crate::diagram_store::{Snapshot, Store};
use crate::label_editor::LabelEditor;
use gpui::{
    Bounds, Context, FocusHandle, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PathBuilder, Pixels, Point, Task, Window, canvas, div, fill, point, prelude::*,
    px, rgb, rgba, size,
};
use std::{cell::Cell, collections::BTreeSet, rc::Rc};

#[derive(Clone, PartialEq)]
enum Selection {
    Blocks(BTreeSet<usize>),
    Port(Port),
    Wire(usize),
}
#[derive(Clone, Copy)]
enum LabelTarget {
    Block(usize),
    Port(Port),
}
struct EditLabel {
    target: LabelTarget,
    input: LabelEditor,
}
pub struct DiagramPrototype {
    diagram: Diagram,
    store: Store,
    save_error: Option<String>,
    routes: Vec<Route>,
    junctions: Vec<Junction>,
    selection: Option<Selection>,
    pending: Option<Port>,
    hovered: Option<Port>,
    drag: Option<GroupDrag>,
    marquee: Option<Marquee>,
    pan_drag: Option<(Point<Pixels>, Pos)>,
    mouse: Pos,
    pan: Pos,
    zoom: f32,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    focus: FocusHandle,
    edit: Option<EditLabel>,
    _blink: Task<()>,
    status: String,
}

impl DiagramPrototype {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        store: Store,
        snapshot: Snapshot,
    ) -> Self {
        let diagram = snapshot.diagram;
        let routes = route_all(&diagram);
        let junctions = junctions(&diagram, &routes);
        let focus = cx.focus_handle();
        window.focus(&focus);
        let blink = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(100))
                    .await;
                if this
                    .update(cx, |this, cx| {
                        if let Some(edit) = &mut this.edit
                            && edit.input.tick()
                        {
                            cx.notify();
                        }
                    })
                    .is_err()
                {
                    break;
                }
            }
        });
        Self {
            diagram,
            store,
            save_error: None,
            routes,
            junctions,
            selection: None,
            pending: None,
            hovered: None,
            drag: None,
            marquee: None,
            pan_drag: None,
            mouse: Pos::default(),
            pan: snapshot.pan,
            zoom: snapshot.zoom,
            bounds: Rc::new(Cell::new(Bounds::default())),
            focus,
            edit: None,
            _blink: blink,
            status: "Click an output square, then an input square to connect.".into(),
        }
    }
    fn world(&self, p: Point<Pixels>) -> Pos {
        let origin = self.bounds.get().origin;
        Pos::new(
            (f32::from(p.x - origin.x) - self.pan.x) / self.zoom,
            (f32::from(p.y - origin.y) - self.pan.y) / self.zoom,
        )
    }
    fn autosave(&mut self) {
        self.save_error = self
            .store
            .save(&self.diagram, self.pan, self.zoom)
            .err()
            .map(|e| format!("AUTOSAVE FAILED: {e}"));
    }
    fn reroute(&mut self) {
        self.routes = route_all(&self.diagram);
        self.junctions = junctions(&self.diagram, &self.routes);
        self.autosave();
    }
    fn selected_block(&self) -> Option<usize> {
        match &self.selection {
            Some(Selection::Blocks(ids)) if ids.len() == 1 => ids.first().copied(),
            Some(Selection::Port(p)) => Some(p.block),
            _ => None,
        }
    }
    fn block_ids(&self) -> BTreeSet<usize> {
        match &self.selection {
            Some(Selection::Blocks(ids)) => ids.clone(),
            _ => BTreeSet::new(),
        }
    }
    fn select_blocks(&mut self, ids: BTreeSet<usize>) {
        self.selection = if ids.is_empty() {
            None
        } else {
            Some(Selection::Blocks(ids))
        };
    }
    fn update_gesture(&mut self, pointer: Pos) {
        if let Some(area) = &mut self.marquee {
            area.end = pointer;
            let ids = area.selected(&self.diagram);
            self.status = format!(
                "{} blocks selected. Only fully enclosed blocks are included.",
                ids.len()
            );
            self.select_blocks(ids);
        }
        if let Some(drag) = &self.drag {
            let positions = drag.positions(pointer);
            let mut changed = false;
            for (id, pos) in positions {
                let b = self.diagram.block_mut(id);
                if b.pos != pos {
                    b.pos = pos;
                    changed = true;
                }
            }
            if changed {
                self.reroute();
            }
        }
    }
    fn release(&mut self, e: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(edit) = &mut self.edit {
            if edit.input.selecting {
                edit.input.mouse_move(e.position);
            }
            edit.input.selecting = false;
            cx.notify();
            return;
        }
        self.mouse = self.world(e.position);
        self.update_gesture(self.mouse);
        self.hovered = self.port_at(e.position);
        if let Some(from) = self.pending
            && let Some(to) = self
                .hovered
                .filter(|&to| self.diagram.can_connect(from, to).is_ok())
        {
            self.diagram
                .connect(from, to)
                .expect("validated drop target");
            self.pending = None;
            self.reroute();
            self.status = "Connected. Drag either block to test the routing.".into();
        }
        self.drag = None;
        self.marquee = None;
        cx.notify();
    }
    fn hit_port(&self, p: Pos) -> Option<Port> {
        for b in self.diagram.blocks.iter().rev() {
            for output in [false, true] {
                let count = if output {
                    b.outputs.len()
                } else {
                    b.inputs.len()
                };
                for index in 0..count {
                    let q = b.port(output, index);
                    if (p.x - q.x).abs() <= 10. / self.zoom && (p.y - q.y).abs() <= 10. / self.zoom
                    {
                        return Some(Port {
                            block: b.id,
                            index,
                            output,
                        });
                    }
                }
            }
        }
        None
    }
    fn port_at(&self, screen: Point<Pixels>) -> Option<Port> {
        if self.bounds.get().contains(&screen) {
            self.hit_port(self.world(screen))
        } else {
            None
        }
    }
    fn down(&mut self, e: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus);
        if self.edit.is_some() {
            self.status = "Finish the label with Enter, or cancel with Escape.".into();
            cx.notify();
            return;
        }
        let p = self.world(e.position);
        self.mouse = p;
        self.drag = None;
        self.marquee = None;
        self.pan_drag = None;
        let hit_port = self.port_at(e.position);
        self.hovered = hit_port;
        if let Some(port) = hit_port.filter(|_| !e.modifiers.shift) {
            self.selection = Some(Selection::Port(port));
            if e.click_count == 2 {
                self.start_edit();
            } else if port.output {
                self.pending = Some(port);
                self.status = format!(
                    "Connecting {}. Drag to an input or click one; Escape cancels.",
                    self.diagram.port_name(port)
                );
            } else if let Some(from) = self.pending {
                match self.diagram.connect(from, port) {
                    Ok(()) => {
                        self.pending = None;
                        self.reroute();
                        self.status = "Connected. Drag either block to test the routing.".into();
                    }
                    Err(message) => self.status = message.into(),
                }
            } else {
                self.status =
                    "Input selected. Start a connection from an output on the right.".into();
            }
        } else if let Some(id) = self
            .diagram
            .blocks
            .iter()
            .rev()
            .find(|b| b.contains(p, 0.))
            .map(|b| b.id)
            .or_else(|| hit_port.map(|port| port.block))
        {
            self.pending = None;
            // A quick click after Shift-selection must still start a group drag.
            if e.click_count == 2 && !e.modifiers.shift && self.block_ids().len() <= 1 {
                self.select_blocks(BTreeSet::from([id]));
                self.start_edit();
            } else {
                let ids = clicked_blocks(&self.block_ids(), id, e.modifiers.shift);
                if ids.contains(&id) {
                    self.drag = Some(GroupDrag::new(p, &ids, &self.diagram));
                }
                self.status = format!(
                    "{} blocks selected. Drag any selected block to move the group.",
                    ids.len()
                );
                self.select_blocks(ids);
            }
        } else {
            self.pending = None;
            let wire = self
                .routes
                .iter()
                .rev()
                .find(|r| wire_distance(p, &r.points) < 7. / self.zoom)
                .map(|r| r.wire);
            if let Some(id) = wire.filter(|_| !e.modifiers.shift) {
                self.selection = Some(Selection::Wire(id));
                self.status = "Wire selected. Delete removes it.".into();
            } else if e.click_count == 2 && !e.modifiers.shift {
                self.add_at(p);
            } else {
                let base = if e.modifiers.shift {
                    self.block_ids()
                } else {
                    BTreeSet::new()
                };
                self.select_blocks(base.clone());
                self.marquee = Some(Marquee {
                    start: p,
                    end: p,
                    base,
                });
                self.status = "Drag a box to enclose blocks. Shift adds to the selection.".into();
            }
        }
        cx.notify();
    }
    fn moved(&mut self, e: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(edit) = &mut self.edit {
            if edit.input.selecting {
                if e.dragging() {
                    edit.input.mouse_move(e.position);
                } else {
                    edit.input.selecting = false;
                }
                cx.notify();
            }
            return;
        }
        self.mouse = self.world(e.position);
        let old_hover = self.hovered;
        self.hovered = self.port_at(e.position);
        if let Some((start, pan)) = self.pan_drag {
            if e.pressed_button == Some(MouseButton::Middle) {
                self.pan = Pos::new(
                    pan.x + f32::from(e.position.x - start.x),
                    pan.y + f32::from(e.position.y - start.y),
                );
                cx.notify();
            } else {
                self.pan_drag = None;
            }
        }
        if self.drag.is_some() || self.marquee.is_some() {
            if e.dragging() {
                self.update_gesture(self.mouse);
            } else {
                self.drag = None;
                self.marquee = None;
            }
            cx.notify();
        }
        self.autosave();
        if self.pending.is_some() || self.hovered != old_hover {
            cx.notify();
        }
    }
    fn add_at(&mut self, p: Pos) {
        let mut pos = Pos::new(p.x.clamp(0., 3300.), p.y.clamp(0., 3300.)).snap();
        // Find a nearby empty slot instead of stacking new blocks on top of one another.
        for _ in 0..300 {
            let occupied = self.diagram.blocks.iter().any(|b| {
                pos.x < b.pos.x + WIDTH + 24.
                    && pos.x + WIDTH + 24. > b.pos.x
                    && pos.y < b.pos.y + b.height() + 24.
                    && pos.y + 96. > b.pos.y
            });
            if !occupied {
                break;
            }
            pos.y += 108.;
            if pos.y > 3300. {
                pos.y = 0.;
                pos.x = (pos.x + 240.).min(3600.);
            }
        }
        let id = self.diagram.add(pos);
        self.select_blocks(BTreeSet::from([id]));
        self.pending = None;
        self.reroute();
        self.status = "Block added. Double-click it or press F2 to rename.".into();
    }
    fn add_port(&mut self, output: bool) {
        let Some(id) = self.selected_block() else {
            self.status = "Select exactly one block to add a port.".into();
            return;
        };
        let b = self.diagram.block_mut(id);
        let ports = if output {
            &mut b.outputs
        } else {
            &mut b.inputs
        };
        if ports.len() >= 6 {
            self.status = "This PoC allows six ports per side.".into();
            return;
        }
        ports.push(format!(
            "{}{}",
            if output { "out" } else { "in" },
            ports.len() + 1
        ));
        self.selection = Some(Selection::Port(Port {
            block: id,
            index: ports.len() - 1,
            output,
        }));
        self.pending = None;
        self.reroute();
        self.start_edit();
    }
    fn delete(&mut self) {
        match self.selection.take() {
            Some(Selection::Blocks(ids)) => {
                for id in ids {
                    self.diagram.delete_block(id);
                }
            }
            Some(Selection::Wire(id)) => self.diagram.wires.retain(|w| w.id != id),
            Some(Selection::Port(p)) => {
                let b = self.diagram.block_mut(p.block);
                if p.output {
                    b.outputs.remove(p.index);
                } else {
                    b.inputs.remove(p.index);
                }
                self.diagram.wires.retain(|w| w.from != p && w.to != p);
                for w in &mut self.diagram.wires {
                    for endpoint in [&mut w.from, &mut w.to] {
                        if endpoint.block == p.block
                            && endpoint.output == p.output
                            && endpoint.index > p.index
                        {
                            endpoint.index -= 1;
                        }
                    }
                }
                self.select_blocks(BTreeSet::from([p.block]));
            }
            None => {
                self.status = "Select a block, port, or wire to delete.".into();
                return;
            }
        }
        self.pending = None;
        self.drag = None;
        self.reroute();
        self.status = "Deleted. This prototype has no undo.".into();
    }
    fn start_edit(&mut self) {
        let target = match &self.selection {
            Some(Selection::Blocks(ids)) if ids.len() == 1 => {
                LabelTarget::Block(*ids.first().unwrap())
            }
            Some(Selection::Port(p)) => LabelTarget::Port(*p),
            _ => {
                self.status = "Select exactly one block or port to rename.".into();
                return;
            }
        };
        let text = match target {
            LabelTarget::Block(id) => self.diagram.block(id).name.clone(),
            LabelTarget::Port(p) => self.diagram.port_name(p).to_owned(),
        };
        self.edit = Some(EditLabel {
            target,
            input: LabelEditor::new(text),
        });
        self.pending = None;
        self.drag = None;
    }
    fn key(&mut self, e: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let k = &e.keystroke;
        if (self.drag.is_some() || self.marquee.is_some()) && k.key != "escape" {
            return;
        }
        if self.edit.is_some() {
            if k.key == "escape" {
                self.edit = None;
            } else if k.key == "enter" {
                let edit = self.edit.take().unwrap();
                let value = edit.input.buffer.text.trim();
                if !value.is_empty() {
                    match edit.target {
                        LabelTarget::Block(id) => self.diagram.block_mut(id).name = value.into(),
                        LabelTarget::Port(p) => {
                            let b = self.diagram.block_mut(p.block);
                            if p.output {
                                b.outputs[p.index] = value.into();
                            } else {
                                b.inputs[p.index] = value.into();
                            }
                        }
                    }
                    self.status = "Label updated.".into();
                } else {
                    self.status = "Empty label ignored.".into();
                }
            } else {
                self.edit.as_mut().unwrap().input.key(k, cx);
            }
        } else {
            match k.key.as_str() {
                "escape" => {
                    self.pending = None;
                    self.drag = None;
                    self.marquee = None;
                    self.selection = None;
                    self.status = "Connection cancelled / selection cleared.".into();
                }
                "f2" => self.start_edit(),
                "delete" | "backspace" => self.delete(),
                "n" if !k.modifiers.control => self.add_at(self.viewport_center()),
                _ => {}
            }
        }
        self.autosave();
        cx.notify();
    }
    fn viewport_center(&self) -> Pos {
        let b = self.bounds.get();
        Pos::new(
            (f32::from(b.size.width) / 2. - self.pan.x) / self.zoom - WIDTH / 2.,
            (f32::from(b.size.height) / 2. - self.pan.y) / self.zoom - 48.,
        )
    }
    fn zoom_at(&mut self, factor: f32, anchor: Pos) {
        let world = Pos::new(
            (anchor.x - self.pan.x) / self.zoom,
            (anchor.y - self.pan.y) / self.zoom,
        );
        self.zoom = (self.zoom * factor).clamp(0.4, 1.8);
        self.pan = Pos::new(
            anchor.x - world.x * self.zoom,
            anchor.y - world.y * self.zoom,
        );
    }
    fn fit(&mut self) {
        if self.diagram.blocks.is_empty() {
            self.zoom = 1.;
            self.pan = Pos::new(36., 24.);
            return;
        }
        let min_x = self
            .diagram
            .blocks
            .iter()
            .map(|b| b.pos.x)
            .fold(f32::INFINITY, f32::min)
            - 60.;
        let min_y = self
            .diagram
            .blocks
            .iter()
            .map(|b| b.pos.y)
            .fold(f32::INFINITY, f32::min)
            - 60.;
        let max_x = self
            .diagram
            .blocks
            .iter()
            .map(|b| b.pos.x + WIDTH)
            .fold(0., f32::max)
            + 60.;
        let max_y = self
            .diagram
            .blocks
            .iter()
            .map(|b| b.pos.y + b.height())
            .fold(0., f32::max)
            + 60.;
        let b = self.bounds.get();
        self.zoom = (f32::from(b.size.width) / (max_x - min_x))
            .min(f32::from(b.size.height) / (max_y - min_y))
            .clamp(0.4, 1.4);
        self.pan = Pos::new(
            (f32::from(b.size.width) - (max_x - min_x) * self.zoom) / 2. - min_x * self.zoom,
            (f32::from(b.size.height) - (max_y - min_y) * self.zoom) / 2. - min_y * self.zoom,
        );
    }
    fn button(
        &self,
        id: &'static str,
        label: &str,
        action: impl Fn(&mut Self) + 'static,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(id)
            .px_3()
            .py_1()
            .rounded_sm()
            .border_1()
            .border_color(rgb(0xc8ced7))
            .bg(rgb(0xffffff))
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0xe7eef9)))
            .child(label.to_owned())
            .on_click(cx.listener(move |this, _, window, cx| {
                window.focus(&this.focus);
                if this.edit.is_none() {
                    action(this);
                    this.autosave();
                }
                cx.notify();
            }))
    }
    fn block_element(&self, b: &Block) -> impl IntoElement {
        let z = self.zoom;
        let selected = match &self.selection {
            Some(Selection::Blocks(ids)) => ids.contains(&b.id),
            Some(Selection::Port(p)) => p.block == b.id,
            _ => false,
        };
        let mut body = div()
            .absolute()
            .left(px(self.pan.x + b.pos.x * z))
            .top(px(self.pan.y + b.pos.y * z))
            .w(px(WIDTH * z))
            .h(px(b.height() * z))
            .border_1()
            .border_color(rgb(if selected { 0x2563eb } else { 0xaaa586 }))
            .bg(rgb(if selected { 0xfff9d9 } else { 0xfffce9 }))
            .shadow_sm()
            // Draw an outer ring instead of changing the layout border width.
            // Keep it at 3 screen pixels even when zoomed out.
            .when(selected, |element| {
                element.child(
                    div()
                        .absolute()
                        .left(px(-3.))
                        .top(px(-3.))
                        .w(px(WIDTH * z + 4.))
                        .h(px(b.height() * z + 4.))
                        .border_3()
                        .border_color(rgb(0x2563eb)),
                )
            })
            .child(
                div()
                    .absolute()
                    .top(px(5. * z))
                    .w_full()
                    .text_center()
                    .text_size(px(10. * z))
                    .text_color(rgb(0x807956))
                    .child(format!("«block»  {}", b.kind)),
            )
            .child(
                div()
                    .absolute()
                    .top(px(21. * z))
                    .px(px(8. * z))
                    .w_full()
                    .overflow_hidden()
                    .text_center()
                    .text_size(px(13. * z))
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(b.name.clone()),
            )
            .child(
                div()
                    .absolute()
                    .top(px(43. * z))
                    .w_full()
                    .h(px(1.))
                    .bg(rgb(0xc8c1a3)),
            );
        for output in [false, true] {
            let ports = if output { &b.outputs } else { &b.inputs };
            for (index, name) in ports.iter().enumerate() {
                let port = Port {
                    block: b.id,
                    index,
                    output,
                };
                let incoming_color = self
                    .diagram
                    .wires
                    .iter()
                    .find(|w| w.to == port)
                    .map(|w| w.color);
                let port_color = incoming_color.unwrap_or(COLORS[b.id % COLORS.len()]);
                let active =
                    self.selection == Some(Selection::Port(port)) || self.pending == Some(port);
                let hovered = self.hovered == Some(port) && self.edit.is_none();
                let valid_target = hovered
                    && self
                        .pending
                        .is_some_and(|from| self.diagram.can_connect(from, port).is_ok());
                let port_size = if hovered { (10. * z).max(16.) } else { 10. * z };
                let y = (60. + index as f32 * 24.) * z;
                body = body
                    .child(
                        div()
                            .absolute()
                            .left(px(if output { (WIDTH - 86.) * z } else { 11. * z }))
                            .top(px(y - 8. * z))
                            .w(px(74. * z))
                            .overflow_hidden()
                            .text_size(px(11. * z))
                            .when(output, |e| e.text_right())
                            .child(name.clone()),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(px(if output { WIDTH * z } else { 0. } - port_size / 2.))
                            .top(px(y - port_size / 2.))
                            .size(px(port_size))
                            .border_1()
                            .when(active || hovered, |element| element.border_2())
                            .border_color(rgb(if valid_target {
                                0x128275
                            } else if active || hovered {
                                0x2563eb
                            } else {
                                port_color
                            }))
                            .bg(rgb(if valid_target {
                                0x9ce5d3
                            } else if hovered || (active && output) {
                                0xfabf36
                            } else if output {
                                port_color
                            } else {
                                // Keep the source color even when this input is selected.
                                incoming_color.unwrap_or(0xffffff)
                            })),
                    );
            }
        }
        body
    }
}

impl Render for DiagramPrototype {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let routes = self.routes.clone();
        let junctions = self.junctions.clone();
        let selected_wire = match &self.selection {
            Some(Selection::Wire(id)) => Some(*id),
            _ => None,
        };
        let pending = self.pending.map(|p| self.diagram.port_pos(p));
        let mouse = self.mouse;
        let pan = self.pan;
        let zoom = self.zoom;
        let bounds_cell = self.bounds.clone();
        let failures: Vec<_> = self
            .routes
            .iter()
            .filter(|r| r.points.is_empty())
            .map(|r| {
                let w = self.diagram.wires.iter().find(|w| w.id == r.wire).unwrap();
                (self.diagram.port_pos(w.from), self.diagram.port_pos(w.to))
            })
            .collect();
        let failed_count = failures.len();
        let drawing = canvas(
            move |bounds, _, _| bounds_cell.set(bounds),
            move |bounds, _, window, _| {
                let screen = |p: Pos| {
                    point(
                        bounds.left() + px(pan.x + p.x * zoom),
                        bounds.top() + px(pan.y + p.y * zoom),
                    )
                };
                // Light dotted grid. Lines and text stay visually dominant.
                let step = GRID * 2. * zoom;
                let mut x = pan.x.rem_euclid(step);
                while x < f32::from(bounds.size.width) {
                    let mut y = pan.y.rem_euclid(step);
                    while y < f32::from(bounds.size.height) {
                        window.paint_quad(fill(
                            Bounds::new(
                                point(bounds.left() + px(x), bounds.top() + px(y)),
                                size(px(1.), px(1.)),
                            ),
                            rgb(0xdde3eb),
                        ));
                        y += step;
                    }
                    x += step;
                }
                for route in routes {
                    if route.points.is_empty() {
                        continue;
                    }
                    // White underlay makes crossings readable without implying a junction.
                    let selected = selected_wire == Some(route.wire);
                    stroke(
                        &route.points,
                        &screen,
                        if selected { 8. } else { 5. },
                        0xffffff,
                        window,
                    );
                    stroke(
                        &route.points,
                        &screen,
                        if selected { 4. } else { 2. },
                        route.color,
                        window,
                    );
                }
                // Paint after every wire so crossing underlays cannot erase branch dots.
                for junction in junctions {
                    let center = screen(junction.position);
                    let radius = px((4. * zoom).clamp(3., 6.));
                    window.paint_quad(
                        fill(
                            Bounds::new(
                                point(center.x - radius, center.y - radius),
                                size(radius * 2., radius * 2.),
                            ),
                            rgb(junction.color),
                        )
                        .corner_radii(radius),
                    );
                }
                for (a, b) in failures {
                    for p in [a, b] {
                        stroke(
                            &[Pos::new(p.x - 7., p.y - 7.), Pos::new(p.x + 7., p.y + 7.)],
                            &screen,
                            3.,
                            0xd32f2f,
                            window,
                        );
                        stroke(
                            &[Pos::new(p.x + 7., p.y - 7.), Pos::new(p.x - 7., p.y + 7.)],
                            &screen,
                            3.,
                            0xd32f2f,
                            window,
                        );
                    }
                }
                if let Some(a) = pending {
                    let path = [
                        a,
                        Pos::new(a.x + 24., a.y),
                        Pos::new(a.x + 24., mouse.y),
                        mouse,
                    ];
                    stroke(&path, &screen, 1.5, 0x8793a6, window);
                }
            },
        )
        .absolute()
        .size_full();
        let mut board = div()
            .id("diagram-canvas")
            .relative()
            .flex_1()
            .min_h_0()
            .w_full()
            .overflow_hidden()
            .bg(rgb(0xffffff))
            .child(drawing)
            .children(self.diagram.blocks.iter().map(|b| self.block_element(b)))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::down))
            .on_mouse_down(
                MouseButton::Middle,
                cx.listener(|this, e: &MouseDownEvent, _, cx| {
                    if this.edit.is_some() || this.drag.is_some() || this.marquee.is_some() {
                        return;
                    }
                    this.pan_drag = Some((e.position, this.pan));
                    cx.notify();
                }),
            )
            .on_scroll_wheel(cx.listener(|this, e: &gpui::ScrollWheelEvent, _, cx| {
                if this.edit.is_some() || this.drag.is_some() || this.marquee.is_some() {
                    return;
                }
                let d = e.delta.pixel_delta(px(24.));
                if e.modifiers.control {
                    let origin = this.bounds.get().origin;
                    this.zoom_at(
                        if d.y > px(0.) { 1.1 } else { 1. / 1.1 },
                        Pos::new(
                            f32::from(e.position.x - origin.x),
                            f32::from(e.position.y - origin.y),
                        ),
                    );
                } else {
                    this.pan.x += f32::from(d.x);
                    this.pan.y += f32::from(d.y);
                }
                this.autosave();
                cx.notify();
            }));
        if let Some(area) = &self.marquee {
            let (min, max) = area.bounds();
            board = board.child(
                div()
                    .absolute()
                    .left(px(pan.x + min.x * zoom))
                    .top(px(pan.y + min.y * zoom))
                    .w(px((max.x - min.x) * zoom))
                    .h(px((max.y - min.y) * zoom))
                    .border_1()
                    .border_color(rgb(0x315bd6))
                    .bg(rgba(0x315bd62b)),
            );
        }
        if let Some(edit) = &self.edit {
            let at = match edit.target {
                LabelTarget::Block(id) => self.diagram.block(id).pos,
                LabelTarget::Port(p) => self.diagram.port_pos(p),
            };
            let target_label = match edit.target {
                LabelTarget::Block(id) => format!("BLOCK · {}", self.diagram.block(id).name),
                LabelTarget::Port(p) => format!(
                    "{} · {} / {}",
                    if p.output { "OUTPUT" } else { "INPUT" },
                    self.diagram.block(p.block).name,
                    self.diagram.port_name(p)
                ),
            };
            let range = edit.input.buffer.selection();
            let hint = if range.is_empty() {
                "Shift+Arrow selects · Ctrl+Arrow moves by word".to_owned()
            } else {
                format!(
                    "{} characters selected · Type to replace",
                    edit.input.buffer.text[range].chars().count()
                )
            };
            let bounds = self.bounds.get();
            board = board.child(
                div()
                    .absolute()
                    .left(px((pan.x + at.x * zoom)
                        .clamp(8., (f32::from(bounds.size.width) - 360.).max(8.))))
                    .top(px((pan.y + at.y * zoom - 116.)
                        .clamp(8., (f32::from(bounds.size.height) - 120.).max(8.))))
                    .w(px(344.))
                    .p_3()
                    .bg(rgb(0xffffff))
                    .border_2()
                    .border_color(rgb(0x315bd6))
                    .shadow_md()
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(0x62718a))
                            .overflow_hidden()
                            .child(target_label),
                    )
                    .child(
                        div()
                            .mt_2()
                            .child(edit.input.element(self.focus.is_focused(window)))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, e: &MouseDownEvent, window, cx| {
                                    window.focus(&this.focus);
                                    if let Some(edit) = &mut this.edit {
                                        edit.input.mouse_down(
                                            e.position,
                                            e.modifiers.shift,
                                            e.click_count,
                                        );
                                    }
                                    cx.stop_propagation();
                                    cx.notify();
                                }),
                            ),
                    )
                    .child(
                        div()
                            .mt_1()
                            .text_size(px(10.))
                            .text_color(rgb(0x506582))
                            .child(hint),
                    )
                    .child(
                        div()
                            .text_size(px(10.))
                            .text_color(rgb(0x506582))
                            .child("Enter saves · Esc cancels"),
                    )
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()),
            );
        }
        let selection = match &self.selection {
            Some(Selection::Blocks(ids)) if ids.len() == 1 => {
                format!("Block: {}", self.diagram.block(*ids.first().unwrap()).name)
            }
            Some(Selection::Blocks(ids)) => format!("{} blocks selected", ids.len()),
            Some(Selection::Port(p)) => format!(
                "{}: {}",
                if p.output { "Output" } else { "Input" },
                self.diagram.port_name(*p)
            ),
            Some(Selection::Wire(id)) => {
                let w = self.diagram.wires.iter().find(|w| w.id == *id).unwrap();
                format!(
                    "{} → {}",
                    self.diagram.port_name(w.from),
                    self.diagram.port_name(w.to)
                )
            }
            None => "Nothing selected".into(),
        };
        div()
            .id("prototype")
            .track_focus(&self.focus)
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0xf4f6f9))
            .text_color(rgb(0x263247))
            .text_size(px(13.))
            .on_key_down(cx.listener(Self::key))
            .on_mouse_move(cx.listener(Self::moved))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::release))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::release))
            .on_mouse_up(
                MouseButton::Middle,
                cx.listener(|this, _, _, _| this.pan_drag = None),
            )
            .on_mouse_up_out(
                MouseButton::Middle,
                cx.listener(|this, _, _, _| this.pan_drag = None),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(rgb(0xd5dce6))
                    .child(
                        div()
                            .mr_3()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Diagram PoC"),
                    )
                    .child(self.button("add", "+ Block", |s| s.add_at(s.viewport_center()), cx))
                    .child(self.button("input", "+ Input", |s| s.add_port(false), cx))
                    .child(self.button("output", "+ Output", |s| s.add_port(true), cx))
                    .child(self.button("rename", "Rename · F2", Self::start_edit, cx))
                    .child(self.button("delete", "Delete", Self::delete, cx))
                    .child(div().flex_1())
                    .child(self.button(
                        "minus",
                        "−",
                        |s| {
                            let b = s.bounds.get();
                            s.zoom_at(
                                1. / 1.2,
                                Pos::new(
                                    f32::from(b.size.width) / 2.,
                                    f32::from(b.size.height) / 2.,
                                ),
                            );
                        },
                        cx,
                    ))
                    .child(format!("{:.0}%", zoom * 100.))
                    .child(self.button(
                        "plus",
                        "+",
                        |s| {
                            let b = s.bounds.get();
                            s.zoom_at(
                                1.2,
                                Pos::new(
                                    f32::from(b.size.width) / 2.,
                                    f32::from(b.size.height) / 2.,
                                ),
                            );
                        },
                        cx,
                    ))
                    .child(self.button("fit", "Fit", Self::fit, cx)),
            )
            .child(board)
            .child(
                div()
                    .px_4()
                    .py_2()
                    .border_t_1()
                    .border_color(rgb(0xd5dce6))
                    .text_size(px(12.))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .gap_3()
                            .child(selection)
                            .child(
                                div()
                                    .text_color(rgb(if failed_count > 0 {
                                        0xc33434
                                    } else {
                                        0x5c6b7e
                                    }))
                                    .child(format!(
                                        "{} blocks  ·  {} wires  ·  {} unroutable",
                                        self.diagram.blocks.len(),
                                        self.diagram.wires.len(),
                                        failed_count
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .mt_1()
                            .text_color(rgb(if self.save_error.is_some() {
                                0xc33434
                            } else {
                                0x62718a
                            }))
                            .child(
                                self.save_error
                                    .clone()
                                    .unwrap_or_else(|| format!("{}  ·  Autosaved", self.status)),
                            ),
                    ),
            )
    }
}

fn stroke(
    points: &[Pos],
    screen: &impl Fn(Pos) -> Point<Pixels>,
    width: f32,
    color: u32,
    window: &mut Window,
) {
    let mut path = PathBuilder::stroke(px(width));
    for (i, &p) in points.iter().enumerate() {
        if i == 0 {
            path.move_to(screen(p));
        } else {
            path.line_to(screen(p));
        }
    }
    if let Ok(path) = path.build() {
        window.paint_path(path, rgb(color));
    }
}
