import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';

interface DraggableItem {
  id: string;
  text: string;
  top: number;
  left: number;
}

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './app.html',
  styleUrls: ['./app.css']
})
export class App {
  title = 'drag-drop-poc';
  droppedItems: string[] = [];
  mode: 'both' | 'drag-only' | 'drop-only' | 'random' = 'both';
  
  randomItems: DraggableItem[] = [];

  ngOnInit() {
    const urlParams = new URLSearchParams(window.location.search);
    const modeParam = urlParams.get('mode') as 'both' | 'drag-only' | 'drop-only' | 'random';
    if (modeParam && ['both', 'drag-only', 'drop-only', 'random'].includes(modeParam)) {
      this.mode = modeParam;
    }
    
    if (this.mode === 'random') {
      this.generateRandomItems();
    }
  }

  setMode(newMode: 'both' | 'drag-only' | 'drop-only' | 'random') {
    this.mode = newMode;
    const url = new URL(window.location.href);
    url.searchParams.set('mode', newMode);
    window.history.pushState({}, '', url.toString());
    
    if (newMode === 'random') {
      this.generateRandomItems();
    }
  }

  generateRandomItems() {
    this.randomItems = Array.from({ length: 15 }).map((_, i) => ({
      id: `rand-item-${i}`,
      text: `Random Item ${i}`,
      top: Math.floor(Math.random() * 60) + 10, // 10% to 70%
      left: Math.floor(Math.random() * 80) + 5, // 5% to 85%
    }));
    
    // Ensure one specific item to find
    this.randomItems.push({
      id: 'target-item',
      text: 'Find Me To Drag',
      top: Math.floor(Math.random() * 60) + 10,
      left: Math.floor(Math.random() * 80) + 5,
    });
  }

  onDragStart(event: DragEvent, item: string) {
    if (event.dataTransfer) {
      event.dataTransfer.setData('text/plain', item);
      event.dataTransfer.effectAllowed = 'move';
    }
  }

  onDragOver(event: DragEvent) {
    event.preventDefault();
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'move';
    }
  }

  onDrop(event: DragEvent) {
    event.preventDefault();
    if (event.dataTransfer) {
      const data = event.dataTransfer.getData('text/plain');
      if (data) {
        this.droppedItems.push(data);
      }
    }
  }
}
