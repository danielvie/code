import React, { useMemo } from 'react';
import { ReactFlow, useNodesState, useEdgesState } from '@xyflow/react';
import NodeCustom from './nodes/NodeCustom';
import '@xyflow/react/dist/style.css';

// Registering the custom node type
const nodeTypes = {
  special: NodeCustom,
};

const initialNodes = [
  { 
    id: '1', 
    type: 'special', // matches the key in nodeTypes
    position: { x: 0, y: 0 }, 
    data: { label: 'Start Node', description: 'Trigger point', icon: '⚡' } 
  },
  { 
    id: '2', 
    type: 'special', 
    position: { x: 300, y: 0 }, 
    data: { label: 'Process', description: 'Computing data...', icon: '⚙️' } 
  },
  { 
    id: '3', 
    type: 'special', 
    position: { x: 600, y: 0 }, 
    data: { label: 'End Node', description: 'Result saved', icon: '💾' } 
  },
];

const initialEdges = [
  { id: 'e1-2', source: '1', target: '2', animated: true },
  { id: 'e2-3', source: '2', target: '3' },
];

export default function App() {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  return (
    <div className="h-screen w-full bg-gray-50">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        fitView
      />
    </div>
  );
}