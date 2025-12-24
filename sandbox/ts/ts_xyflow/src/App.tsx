import React, { useCallback } from 'react';
import { ReactFlow, useNodesState, useEdgesState, addEdge, Background, Controls } from '@xyflow/react';
import CustomNode from './components/CustomNode';

const nodeTypes = {
  custom: CustomNode,
};

const initialNodes = [
  { 
    id: '1', 
    type: 'custom', 
    position: { x: 50, y: 100 }, 
    data: { label: 'Step 1', description: 'Initial Trigger', icon: '⚡' } 
  },
  { 
    id: '2', 
    type: 'custom', 
    position: { x: 300, y: 100 }, 
    data: { label: 'Step 2', description: 'Processing...', icon: '⚙️' } 
  },
  { 
    id: '3', 
    type: 'custom', 
    position: { x: 550, y: 100 }, 
    data: { label: 'Step 3', description: 'Final Result', icon: '✅' } 
  },
];

const initialEdges = [
  { id: 'e1-2', source: '1', target: '2', animated: true },
  { id: 'e2-3', source: '2', target: '3' },
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
        <Background color="#cbd5e1" variant="dots" />
        <Controls />
      </ReactFlow>
    </div>
  );
}