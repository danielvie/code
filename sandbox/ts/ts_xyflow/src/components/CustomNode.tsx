import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';

export default memo(({ data, selected }) => {
  return (
    <div className={`
      group relative bg-white rounded-lg border-2 transition-all p-3 min-w-[150px]
      ${selected ? 'border-blue-500 shadow-lg' : 'border-slate-200 shadow-sm'}
    `}>
      {/* Target Handle (Input) */}
      <Handle 
        type="target" 
        position={Position.Left} 
        className="!bg-slate-400 !w-2 !h-2 border-2 border-white" 
      />

      <div className="flex flex-col gap-1">
        <div className="flex items-center gap-2">
          <span className="text-lg">{data.icon}</span>
          <h3 className="font-bold text-slate-700 text-sm">{data.label}</h3>
        </div>
        <p className="text-xs text-slate-500">{data.description}</p>
      </div>

      {/* Source Handle (Output) */}
      <Handle 
        type="source" 
        position={Position.Right} 
        className="!bg-blue-500 !w-2 !h-2 border-2 border-white" 
      />
    </div>
  );
});