import { useEffect, useState } from "react";
import { Scene } from "../../data/Scene";
import { tauri } from "@tauri-apps/api";
import { ExecuteSceneButton } from "../ExecuteSceneButton";

// デバッグ用にテキストを返却
const SceneTab = () => {
    const [scenes, setScenes] = useState<Scene[]>([]);
    // インスタンス化されたときに一度だけログを吐く
    useEffect(() => {
        tauri.invoke("get_scenes").then((res) => {
            const resDatas : Scene[] = JSON.parse(JSON.stringify(res));
            setScenes(resDatas);
        });
    }, []);


    return (
        // シーンデータを画面内に並べる
        <div className="py-5 grid grid-cols-4 justify-center">
            {/* ExecuteSceneButtonを並べる */}
            {scenes.map((scene, index) => (
                <ExecuteSceneButton key={index} data={scene}/>
            ))}
        </div>
    );
}

export default SceneTab;