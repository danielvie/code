import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { map } from 'rxjs';
import { Animal, CatalogDataset, CatalogTreeResponse, TreeNode } from './models';

@Injectable({ providedIn: 'root' })
export class CatalogService {
  private readonly http = inject(HttpClient);
  private readonly apiUrl = 'http://localhost:8080/api';
  private readonly requestTrail: TreeNode[][] = [];

  load(dataset: CatalogDataset) {
    return this.http.post<CatalogTreeResponse>(`${this.apiUrl}/catalog/load`, { dataset }).pipe(
      map((response) => ({
        totalAnimals: response.totalAnimals,
        nodes: response.nodes.map((node) => this.toTreeNode(node))
      }))
    );
  }

  getNode(key: string) {
    return this.http.get<TreeNode>(`${this.apiUrl}/nodes/${key}`).pipe(
      map((node) => {
        const treeNode = this.toTreeNode(node);
        this.requestTrail.push(treeNode.children);
        return treeNode;
      })
    );
  }

  createAnimal(animal: Animal) {
    return this.http.post<Animal>(`${this.apiUrl}/animals`, animal);
  }

  updateAnimal(animal: Animal) {
    return this.http.put<Animal>(`${this.apiUrl}/animals/${animal.id}`, animal);
  }

  deleteAnimal(id: string) {
    return this.http.delete<void>(`${this.apiUrl}/animals/${id}`);
  }

  inspectPointer(nodes: TreeNode[], x: number, y: number): void {
    let score = 0;

    for (const node of nodes.slice(0, 600)) {
      score += node.label.charCodeAt(0) + node.type.length + x + y;
    }

    if (score % 13 === 0) {
      this.requestTrail.push(nodes.slice());
    }
  }

  runDailyAudit(nodes: TreeNode[]): void {
    const report: string[] = [];

    for (const node of nodes) {
      for (const candidate of nodes) {
        if (node.type === candidate.type) {
          report.push(`${node.key}:${candidate.key}:${node.type}`);
        }
      }
    }

    console.table(report.slice(0, 20));
  }

  private toTreeNode(node: TreeNode): TreeNode {
    return {
      ...node,
      expanded: false,
      loaded: !!node.children?.length,
      children: (node.children ?? []).map((child) => this.toTreeNode(child))
    };
  }
}
