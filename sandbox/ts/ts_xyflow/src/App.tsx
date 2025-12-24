import Flow from './components/Flow';
import { get_data } from './services/getters';

const data = get_data()

export default function App() {

  // const onConnect = useCallback((params) => setEdges((eds) => addEdge(params, eds)), []);

  return (
    <div className="w-full h-screen bg-slate-50">
      <Flow data={data}></Flow>
    </div>
  );
}