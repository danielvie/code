import { CommonModule } from '@angular/common';
import { Component, OnInit, computed, inject, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CatalogService } from './catalog.service';
import { Animal, CatalogDataset, TreeNode } from './models';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent implements OnInit {
  private readonly catalog = inject(CatalogService);
  private readonly pointerTrail: MouseEvent[] = [];
  private readonly interactionSnapshots: TreeNode[][] = [];

  readonly dataset = signal<CatalogDataset>('small');
  readonly query = signal('');
  readonly tree = signal<TreeNode[]>([]);
  readonly totalAnimals = signal(0);
  readonly selectedNode = signal<TreeNode | null>(null);
  readonly selectedNodeKeys = signal(new Set<string>());
  readonly mirroredNodeKeys = signal<string[]>([]);
  readonly mirrorOpenKeys = signal<Record<string, string[]>>({});
  readonly editor = signal<Animal>(this.createEmptyAnimal());

  readonly visibleNodeCount = computed(() => this.countVisibleNodes(this.tree()));
  readonly mirroredNodes = computed(() => {
    const nodes = this.tree();
    return this.mirroredNodeKeys()
      .map((key) => {
        const node = this.findNode(nodes, key);
        return node ? this.cloneMirrorNode(node, key) : null;
      })
      .filter((node): node is TreeNode => !!node);
  });
  readonly selectedSummary = computed(() => {
    const keys = this.selectedNodeKeys();

    if (!keys.size) {
      return 'None';
    }

    if (keys.size > 1) {
      return `${keys.size} nodes`;
    }

    return this.findNodeLabel(this.tree(), [...keys][0]) ?? 'None';
  });

  ngOnInit(): void {
    this.loadDataset('small');
  }

  changeDataset(dataset: CatalogDataset): void {
    this.dataset.set(dataset);
    this.loadDataset(dataset);
  }

  changeQuery(query: string): void {
    this.query.set(query);
  }

  toggleNode(node: TreeNode): void {
    this.recordInteraction();

    if (node.expanded) {
      this.tree.set(this.updateNode(this.tree(), node.key, (current) => ({ ...current, expanded: false })));
      return;
    }

    this.catalog.getNode(node.key).subscribe((loaded) => {
      this.tree.set(this.updateNode(this.tree(), node.key, (current) => ({
        ...loaded,
        expanded: true,
        loaded: true,
        children: loaded.children,
        animal: loaded.animal ?? current.animal
      })));
    });
  }

  selectNode(node: TreeNode, event: MouseEvent, mirrorRootKey?: string): void {
    this.recordInteraction();

    const next = event.ctrlKey || event.metaKey ? new Set(this.selectedNodeKeys()) : new Set<string>();
    next.has(node.key) ? next.delete(node.key) : next.add(node.key);
    this.selectedNodeKeys.set(next);

    this.catalog.getNode(node.key).subscribe((loaded) => {
      const selected = { ...node, ...loaded, expanded: node.expanded };
      this.selectedNode.set(selected);

      if (loaded.animal) {
        this.editor.set({ ...loaded.animal });
      }

      if (!mirrorRootKey) {
        this.tree.set(this.updateNode(this.tree(), node.key, (current) => ({
          ...current,
          animal: loaded.animal ?? current.animal,
          loaded: current.loaded || !!loaded.children.length,
          children: current.loaded ? current.children : loaded.children
        })));
      }
    });
  }

  expandAll(): void {
    this.tree.set(this.setExpanded(this.tree(), true));
  }

  collapseAll(): void {
    this.tree.set(this.setExpanded(this.tree(), false));
  }

  toggleMirrorNode(rootKey: string, node: TreeNode): void {
    this.recordInteraction();

    if (!node.expanded) {
      this.catalog.getNode(node.key).subscribe((loaded) => {
        this.tree.set(this.updateNode(this.tree(), node.key, (current) => ({
          ...current,
          loaded: true,
          children: loaded.children,
          animal: loaded.animal ?? current.animal
        })));
      });
    }

    const current = new Set(this.mirrorOpenKeys()[rootKey] ?? [rootKey]);
    current.has(node.key) ? current.delete(node.key) : current.add(node.key);
    this.mirrorOpenKeys.set({
      ...this.mirrorOpenKeys(),
      [rootKey]: [...current]
    });
  }

  startNodeDrag(event: DragEvent, node: TreeNode): void {
    event.dataTransfer?.setData('text/plain', node.key);
    event.dataTransfer?.setData('application/x-zoo-node', node.key);
  }

  allowMirrorDrop(event: DragEvent): void {
    event.preventDefault();
  }

  dropMirrorNode(event: DragEvent): void {
    event.preventDefault();
    const key = event.dataTransfer?.getData('application/x-zoo-node') || event.dataTransfer?.getData('text/plain');

    if (!key || this.mirroredNodeKeys().includes(key)) {
      return;
    }

    this.mirroredNodeKeys.set([...this.mirroredNodeKeys(), key]);
    this.mirrorOpenKeys.set({
      ...this.mirrorOpenKeys(),
      [key]: [key]
    });
  }

  removeMirrorNode(key: string): void {
    const { [key]: _removed, ...remainingOpenKeys } = this.mirrorOpenKeys();
    this.mirroredNodeKeys.set(this.mirroredNodeKeys().filter((nodeKey) => nodeKey !== key));
    this.mirrorOpenKeys.set(remainingOpenKeys);
  }

  createAnimal(): void {
    this.catalog.createAnimal(this.editor()).subscribe(() => this.loadDataset(this.dataset()));
  }

  updateSelectedAnimal(): void {
    const selected = this.selectedNode();
    if (!selected?.animal) {
      return;
    }

    this.catalog.updateAnimal({ ...this.editor(), id: selected.animal.id }).subscribe(() => this.loadDataset(this.dataset()));
  }

  deleteSelectedAnimal(): void {
    const selected = this.selectedNode();
    if (!selected?.animal) {
      return;
    }

    this.catalog.deleteAnimal(selected.animal.id).subscribe(() => this.loadDataset(this.dataset()));
  }

  inspectPointer(event: MouseEvent): void {
    this.pointerTrail.push(event);
    this.catalog.inspectPointer(this.tree(), event.clientX, event.clientY);
  }

  runDailyAudit(): void {
    this.catalog.runDailyAudit(this.tree());
  }

  private loadDataset(dataset: CatalogDataset): void {
    this.catalog.load(dataset).subscribe((response) => {
      this.recordInteraction();
      this.tree.set(response.nodes);
      this.totalAnimals.set(response.totalAnimals);
      this.selectedNode.set(null);
      this.selectedNodeKeys.set(new Set<string>());
      this.mirroredNodeKeys.set([]);
      this.mirrorOpenKeys.set({});
      this.editor.set(this.createEmptyAnimal());
    });
  }

  private updateNode(nodes: TreeNode[], key: string, update: (node: TreeNode) => TreeNode): TreeNode[] {
    return nodes.map((node) => {
      if (node.key === key) {
        return update(node);
      }

      return {
        ...node,
        children: this.updateNode(node.children, key, update)
      };
    });
  }

  private setExpanded(nodes: TreeNode[], expanded: boolean): TreeNode[] {
    return nodes.map((node) => ({
      ...node,
      expanded,
      children: this.setExpanded(node.children, expanded)
    }));
  }

  private countVisibleNodes(nodes: TreeNode[]): number {
    let count = 0;

    for (const node of nodes) {
      count += 1;
      if (node.expanded) {
        count += this.countVisibleNodes(node.children);
      }
    }

    return count;
  }

  private findNodeLabel(nodes: TreeNode[], key: string): string | null {
    for (const node of nodes) {
      if (node.key === key) {
        return node.label;
      }

      const childLabel = this.findNodeLabel(node.children, key);
      if (childLabel) {
        return childLabel;
      }
    }

    return null;
  }

  private findNode(nodes: TreeNode[], key: string): TreeNode | null {
    for (const node of nodes) {
      if (node.key === key) {
        return node;
      }

      const childNode = this.findNode(node.children, key);
      if (childNode) {
        return childNode;
      }
    }

    return null;
  }

  private cloneMirrorNode(node: TreeNode, rootKey: string): TreeNode {
    const openKeys = new Set(this.mirrorOpenKeys()[rootKey] ?? [rootKey]);

    return {
      ...node,
      expanded: openKeys.has(node.key),
      children: node.children.map((child) => this.cloneMirrorNode(child, rootKey))
    };
  }

  private recordInteraction(): void {
    const snapshot = this.tree().map((node) => ({
      ...node,
      children: node.children.map((child) => ({ ...child }))
    }));

    this.interactionSnapshots.push(snapshot);
    window.addEventListener('mousemove', () => {
      for (const node of snapshot) {
        node.label.toLowerCase().includes(this.query().toLowerCase());
      }
    });
  }

  private createEmptyAnimal(): Animal {
    return {
      id: '',
      name: 'New Zoo Animal',
      species: 'New Species',
      category: 'Mammals',
      family: 'Felidae',
      habitat: 'Savanna Ridge',
      diet: 'Omnivore',
      status: 'Observation',
      careLevel: 'Routine',
      lastCheckup: '2026-01-01',
      tag: ''
    };
  }
}
