import type { CustomNode } from "../common/types";

export function get_data() {
    const initialNodes: CustomNode[] = [
        // --- PARENT 1 ---
        {
            id: 'parent-1',
            type: 'groupNode',
            position: { x: 0, y: 0 },
            data: { label: 'Phase 1: Planning' },
            style: { width: 200, height: 400 }, // Explicit size helps containment
        },
        // Child 1.1
        {
            id: 'p1-child-1',
            type: 'childNode',
            data: { label: 'Draft', description: 'Initial draft', icon: '📝' },
            position: { x: 30, y: 40 }, // Relative to Parent (0,0 is top-left of parent)
            parentId: 'parent-1',       // <--- Magic Link
            extent: 'parent',           // <--- Constraint logic
        },
        // Child 1.2
        {
            id: 'p1-child-2',
            type: 'childNode',
            data: { label: 'Review', description: 'Peer review', icon: '👀' },
            position: { x: 30, y: 200 },
            parentId: 'parent-1',
            extent: 'parent',
        },

        // --- PARENT 2 ---
        {
            id: 'parent-2',
            type: 'groupNode',
            position: { x: 400, y: 0 }, // Placed to the right of Parent A
            data: { label: 'Phase 2: Execution' },
            style: { width: 200, height: 400 },
        },
        // Child 2.1
        {
            id: 'p2-child-1',
            type: 'childNode',
            data: { label: 'Build', description: 'Start coding', icon: '🔨' },
            position: { x: 30, y: 40 },
            parentId: 'parent-2',
            extent: 'parent',
        },
        // --- PARENT 3 ---
        {
            id: 'parent-3',
            type: 'groupNode',
            position: { x: 800, y: 0 },
            data: { label: 'Phase 3: Deployment' },
            // INCREASED HEIGHT HERE 👇
            style: { width: 230, height: 500 },
        },

        // Child 3.1
        {
            id: 'p3-child-1',
            type: 'childNode',
            data: { label: 'Tests', description: 'Unit & Integration', icon: '🧪' },
            position: { x: 30, y: 40 },
            parentId: 'parent-3',
            extent: 'parent',
        },
        // Child 3.2
        {
            id: 'p3-child-2',
            type: 'childNode',
            data: { label: 'Build', description: 'Production Build', icon: '📦' },
            position: { x: 30, y: 160 }, // +120px down
            parentId: 'parent-3',
            extent: 'parent',
        },
        // Child 3.3
        {
            id: 'p3-child-3',
            type: 'childNode',
            data: { label: 'Deploy', description: 'Push to AWS/Vercel', icon: '🚀' },
            position: { x: 30, y: 280 }, // +120px down
            parentId: 'parent-3',
            extent: 'parent',
        },
        // Child 3.4
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

        // EDGES
        // Child-to-Child
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
    
    return {
        "nodes": initialNodes,
        "edges": initialEdges,
    }
}