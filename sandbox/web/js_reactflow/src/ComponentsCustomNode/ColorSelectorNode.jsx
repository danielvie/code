import React, { memo } from "react";
import { Handle, Position } from "@xyflow/react";

export default memo(({ data, isConnectable }) => {
  return (
    <div className="flex items-center">
      <div className=" ">
        <Handle
          type="target"
          position={Position.Left}
          onConnect={(params) => console.log("handle onConnect", params)}
          isConnectable={isConnectable}
        />
        <div>
          Custom Color Picker Node: <strong>{data.color}</strong>
        </div>

        <input
          className="nodrag"
          type="color"
          onChange={data.onChange}
          defaultValue={data.color}
        />
        <Handle
          type="source"
          position={Position.Right}
          isConnectable={isConnectable}
        />
      </div>

      <div className="MijnDrag w-5 h-5 bg-sky-700 rounded-full"></div>
    </div>
  );
});
