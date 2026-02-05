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
└── test_page.html

---

## Requirements

- Java JDK 17
- Maven 3.6+
- SikuliX 2.0.5

## How the Automation Works

The DragDropBot supports two automation modes.

### 1. Image-Based Drag & Drop

- Uses template_drag.png and template_drop.png
- Matches elements visually using similarity thresholds
- Best for icons, buttons, and non-text UI

Method used:

runImageBasedDragDrop()

---

### 2. Text-Based Drag & Drop (OCR)

- Uses SikuliX OCR to locate text on screen
- Offsets can be applied for precise drop locations
- Best for labels, tables, and dynamic UI text

Method used:

runTextBasedDragDrop()

---

## Test Page

The test_page.html file provides a simple drag-and-drop UI for testing.

Behavior:
- A green draggable box
- A blue drop zone
- Both elements randomly reposition themselves
- Success is detected via collision logic

Open test_page.html in a browser before running the bot.

---

## Running the Project

### Option 1: Run Using Maven (Recommended)

mvn compile exec:java -Dexec.mainClass="DragDropBot"

Using Taskfile alias:

task mv
