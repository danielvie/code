# Drag & Drop Automation Bot (SikuliX)

This project demonstrates UI automation using SikuliX 2.0.5 to perform
drag-and-drop actions on screen elements using image recognition and OCR.

It can automate desktop applications or browser-based UIs, such as the
included HTML test page.

---

## Features

- Image-based drag & drop using screenshot templates
- Text-based drag & drop using OCR
- Debug output with screen coordinates
- Visual highlights for matched elements
- Java 17 compatible

---

## Project Structure

.
├── pom.xml
├── README.md
├── src
│  └── main
│     └── java
│        ├── DragDropBot.class
│        └── DragDropBot.java
├── Taskfile.yml
├── template_drag.png
├── template_drop.png
├── debug_view.png
├── test_page.html
└── java_robot.zip

Notes:
- `DragDropBot.class` is a compiled artifact checked into `src/main/java`. It is not required to build/run via Maven.

---

## Requirements

- Java JDK 17
- Maven 3.6+
- SikuliX 2.0.5 (`com.sikulix:sikulixapi:2.0.5`)

---

## How the Automation Works

`DragDropBot` uses SikuliX to find UI elements on the current screen and perform a drag-and-drop via `Screen.dragDrop(...)`.

The bot supports two automation modes:

### 1) Image-Based Drag & Drop (Template Matching)

- Uses `template_drag.png` and `template_drop.png`
- Searches the desktop for visual matches using similarity thresholds (currently `0.80`)
- Highlights matches briefly and prints basic status
- Best for icons, buttons, and non-text UI

Method:
- `runImageBasedDragDrop()`

### 2) Text-Based Drag & Drop (OCR)

- Uses SikuliX OCR via `screen.find("<text>")` to locate text on screen
- Highlights matches and prints match coordinates (X/Y/Width/Height/Center)
- Uses configurable offsets to compute exact drag/drop points

Method:
- `runTextBasedDragDrop()`

Current defaults in code:
- Source text: `ACTUATOR-SYS-1`
- Target text: `Working Set Info`
- Target offset: `+100` on Y (drops below the matched target text)

---

## Switching Modes

In `DragDropBot.main(...)`, you enable one mode by calling:
- `runImageBasedDragDrop()` (currently commented out), or
- `runTextBasedDragDrop()` (currently enabled)

---

## Test Page

`test_page.html` provides a simple drag-and-drop UI for testing image-based automation.

Behavior:
- A green draggable box (“DRAG ME”)
- A blue drop zone (“DROP ZONE”)
- Both elements randomly reposition
- Success is detected via collision logic and resets after a short delay

Usage:
1. Open `test_page.html` in a browser.
2. Ensure the window is visible (not minimized/covered).
3. Run the bot using the image-based mode with the provided templates.

---

## Running the Project

### Option 1: Run Using Maven (Recommended)

From the project directory:

mvn compile exec:java -Dexec.mainClass="DragDropBot"

Or via Taskfile alias:

task mv

### Option 2: Run Using Taskfile (local jar)

The `task main` / `task m` task compiles and runs using a locally available `sikulix.jar` on the classpath:

- `task m`

This path assumes `sikulix.jar` is present alongside the project (it is not provided by Maven and may not exist in this repository checkout).

---

## Troubleshooting Tips

- SikuliX operates on what’s actually visible on the screen. Keep the target UI unobstructed.
- If image matching fails, re-capture templates (`template_drag.png`, `template_drop.png`) for your display scaling/theme and adjust similarity thresholds in code.
- If OCR matching fails, confirm the exact on-screen text and consider applying offsets or switching to image templates for more reliability.
