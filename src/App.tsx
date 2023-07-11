import "./App.css";
import InfraredDeviceTab from "./componets/tabs/InfraredDeviceTab";
import SceneTab from "./componets/tabs/SceneTab";
import SettingTab from "./componets/tabs/SettingTab";
import { useState } from "react";

function App() {
  const [activeTab, setActiveTab] = useState(0);

  // 明示的にコンポーネントをインスタンス化することで切り替え時にuseeffectが発火するのを防ぐ
  const scene = SceneTab();
  const infrared = InfraredDeviceTab();
  const setting = SettingTab();
  const tabs = [
    { title: "scene", content: scene},
    { title: "infrared", content: infrared},
    { title: "setting", content: setting},
  ];

  return (
    <div className="flex flex-col bg-gray h-screen">
      {/* nav bar */}
      <div className="h-15 bg-white">
        <nav className="flex">
          {tabs.map((tab, index) => (
            <button
              key={index}
              className={`${
                index === activeTab ? 'text-blue-500 border-b-2 font-medium border-blue-500' : ''
              } text-gray-600 py-4 px-6 block hover:text-blue-500 focus:outline-none`}
              onClick={() => setActiveTab(index)}
            >
              {tab.title}
            </button>
          ))}
        </nav>
      </div>

      {/* content */}
      <div className="flex h-full w-full bg-gray-100 rounded overflow-y-auto">
        {tabs[activeTab].content}
      </div>
    </div>
  );
};

export default App;
