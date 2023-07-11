import "./App.css";
import { ExecuteSceneButton } from "./componets/ExecuteSceneButton";
import { Scene } from "./data/Scene";

function App() {
  const sampledata : Scene = {
    scene_id: "***",
    scene_name: "睡眠"
  };

  return (
    <div className="container">
      <ExecuteSceneButton data={sampledata} />
    </div>
  );
}

export default App;
