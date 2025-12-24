import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';

export default memo(({ data, selected }) => {
  return (
    // ADDED: h-full w-full
    <div className={`
      h-full w-full 
      relative min-w-[250px] min-h-[400px] bg-slate-50/50 rounded-xl 
      border-2 border-dashed transition-all
      ${selected ? 'border-blue-400 bg-blue-50/30' : 'border-slate-300'}
    `}>
      {/* Parent Handle: Input (Left) */}
      <Handle 
        type="target" 
        position={Position.Left} 
        className="!w-4 !h-4 !bg-slate-500 !-ml-2 border-2 border-white"
      />

      <div className="absolute -top-4 left-4 px-3 py-1 bg-slate-100 border border-slate-300 rounded-full shadow-sm">
        <span className="font-bold text-xs text-slate-600 uppercase tracking-wider">
          {data.label}
        </span>
      </div>

      {/* Parent Handle: Output (Right) */}
      <Handle 
        type="source" 
        position={Position.Right} 
        className="!w-4 !h-4 !bg-slate-500 !-mr-2 border-2 border-white"
      />
    </div>
  );
});