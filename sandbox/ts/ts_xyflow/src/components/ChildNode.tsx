import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { MyNodeData } from '../common/types';

interface ChildNodeProps {
  data: MyNodeData;
  selected: boolean;
}

export default memo(({ data, selected }: ChildNodeProps) => {
  return (
    <div className={`
      group relative bg-white rounded-lg border-2 transition-all p-3 min-w-37.5 shadow-sm
      ${selected ? 'border-blue-500 shadow-lg' : 'border-slate-200'}
    `}>
      {/* Target Handle (Top) for incoming vertical connections */}
      <Handle 
        type="target" 
        position={Position.Top} 
        className="bg-white! w-3! h-3! border-2! border-sky-500!" 
      />

      <div className="flex flex-col gap-1">
        <div className="flex items-center gap-2">
          <span className="text-lg">{data.icon}</span>
          <h3 className="font-bold text-slate-700 text-sm">{data.label}</h3>
        </div>
        <p className="text-xs text-slate-500">{data.description}</p>
      </div>

      {/* Source Handle (Bottom) for outgoing vertical connections */}
      <Handle 
        type="source" 
        position={Position.Bottom} 
        className="bg-white! w-3! h-3! border-2! border-green-500!" 
      />
    </div>
  );
});