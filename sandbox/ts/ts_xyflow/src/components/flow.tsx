import { ReactFlow, useNodesState, useEdgesState, Background, Controls, type Node, type Edge } from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import GroupNode from './GroupNode';
import ChildNode from './ChildNode';

const nodeTypes = {
  groupNode: GroupNode,
  childNode: ChildNode,
};

interface FlowProps {
  data: {
    nodes: Node[];
    edges: Edge[];
  };
}

export default function Flow({ data }: FlowProps) {
  const [nodes, , onNodesChange] = useNodesState(data.nodes);
  const [edges, , onEdgesChange] = useEdgesState(data.edges);

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      nodeTypes={nodeTypes}
      onNodesChange={onNodesChange}
      onEdgesChange={onEdgesChange}
      fitView
    >
      <Background />
      <Controls />
    </ReactFlow>
  );
}