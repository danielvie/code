import React, { useCallback } from 'react';
import { ReactFlow, useNodesState, useEdgesState, addEdge, Background, Controls } from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import GroupNode from './components/GroupNode';
import ChildNode from './components/ChildNode';

const nodeTypes = {
  groupNode: GroupNode,
  childNode: ChildNode,
};

const initialNodes = [
  // --- PARENT A ---
  {
    id: 'parent-1',
    type: 'groupNode',
    position: { x: 0, y: 0 },
    data: { label: 'Phase 1: Planning' },
    style: { width: 280, height: 400 }, // Explicit size helps containment
  },
  // Child 1 (Inside Parent A)
  {
    id: 'p1-child-1',
    type: 'childNode',
    data: { label: 'Draft', description: 'Initial draft', icon: '📝' },
    position: { x: 30, y: 40 }, // Relative to Parent (0,0 is top-left of parent)
    parentId: 'parent-1',       // <--- Magic Link
    extent: 'parent',           // <--- Constraint logic
  },
  // Child 2 (Inside Parent A)
  {
    id: 'p1-child-2',
    type: 'childNode',
    data: { label: 'Review', description: 'Peer review', icon: '👀' },
    position: { x: 30, y: 200 },
    parentId: 'parent-1',
    extent: 'parent',
  },

  // --- PARENT B ---
  {
    id: 'parent-2',
    type: 'groupNode',
    position: { x: 400, y: 0 }, // Placed to the right of Parent A
    data: { label: 'Phase 2: Execution' },
    style: { width: 280, height: 400 },
  },
  // Child 1 (Inside Parent B)
  {
    id: 'p2-child-1',
    type: 'childNode',
    data: { label: 'Build', description: 'Start coding', icon: '🔨' },
    position: { x: 30, y: 40 }, 
    parentId: 'parent-2',
    extent: 'parent',
  },
  // --- PARENT C ---
  {
    id: 'parent-3',
    type: 'groupNode',
    position: { x: 800, y: 0 }, 
    data: { label: 'Phase 3: Deployment' },
    // INCREASED HEIGHT HERE 👇
    style: { width: 280, height: 500 }, 
  },

  // Child 1
  {
    id: 'p3-child-1',
    type: 'childNode',
    data: { label: 'Tests', description: 'Unit & Integration', icon: '🧪' },
    position: { x: 30, y: 40 },
    parentId: 'parent-3',
    extent: 'parent',
  },
  // Child 2
  {
    id: 'p3-child-2',
    type: 'childNode',
    data: { label: 'Build', description: 'Production Build', icon: '📦' },
    position: { x: 30, y: 160 }, // +120px down
    parentId: 'parent-3',
    extent: 'parent',
  },
  // Child 3
  {
    id: 'p3-child-3',
    type: 'childNode',
    data: { label: 'Deploy', description: 'Push to AWS/Vercel', icon: '🚀' },
    position: { x: 30, y: 280 }, // +120px down
    parentId: 'parent-3',
    extent: 'parent',
  },
  // Child 4
  {
    id: 'p3-child-4',
    type: 'childNode',
    data: { label: 'Verify', description: 'Health Check', icon: '🩺' },
    position: { x: 30, y: 400 }, // +120px down
    parentId: 'parent-3',
    extent: 'parent',
  },
];

const initialEdges = [
  // Connedt Parent 1 -> Parent 2
  { 
    id: 'e-group', 
    source: 'parent-1', 
    target: 'parent-2', 
    style: { stroke: '#94a3b8', strokeWidth: 2, strokeDasharray: '5,5' } 
  },

  // Child-to-Child (Sequence inside Parent 1)
  { id: 'e1', source: 'p1-child-1', target: 'p1-child-2', animated: true },

  // Connect Parent 2 -> Parent 3
  { 
    id: 'e-group-2-3', 
    source: 'parent-2', 
    target: 'parent-3', 
    style: { stroke: '#94a3b8', strokeWidth: 2, strokeDasharray: '5,5' },
    animated: true 
  },

  // Sequence inside Parent 3
  { id: 'e-p3-1-2', source: 'p3-child-1', target: 'p3-child-2' },
  { id: 'e-p3-2-3', source: 'p3-child-2', target: 'p3-child-3' },
  { id: 'e-p3-3-4', source: 'p3-child-3', target: 'p3-child-4' },
];

export default function App() {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback((params) => setEdges((eds) => addEdge(params, eds)), []);

  return (
    <div className="w-full h-screen bg-slate-50">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        fitView
      >
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
}