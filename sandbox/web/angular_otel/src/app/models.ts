export type CatalogDataset = 'small' | 'large';

export interface Animal {
  id: string;
  name: string;
  species: string;
  category: string;
  family: string;
  habitat: string;
  diet: string;
  status: string;
  careLevel: string;
  lastCheckup: string;
  tag: string;
}

export interface TreeNode {
  key: string;
  label: string;
  type: string;
  count: number;
  hasChildren: boolean;
  expanded: boolean;
  loaded: boolean;
  children: TreeNode[];
  animal?: Animal | null;
}

export interface CatalogTreeResponse {
  nodes: TreeNode[];
  totalAnimals: number;
}
