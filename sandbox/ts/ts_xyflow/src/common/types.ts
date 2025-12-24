import type { Node } from '@xyflow/react';

// Define the shape of your custom data
export type MyNodeData = {
  label: string;
  description?: string; // Make optional if not all nodes have it
  icon?: string;
};

// Create a single type for all your nodes
export type CustomNode = Node<MyNodeData>;