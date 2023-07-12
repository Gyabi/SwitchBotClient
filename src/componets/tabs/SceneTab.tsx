import { useEffect, useState } from "react";
import { Scene } from "../../data/Scene";
import { tauri } from "@tauri-apps/api";
import { ExecuteSceneButton } from "../ExecuteSceneButton";
import MessageDialog from "../MessageDialog";

const SceneTab = () => {
    const [scenes, setScenes] = useState<Scene[]>([]);
    // ダイアログ制御用変数
    const [isMessageDialogOpen, setIsMessageDialogOpen] = useState(false);
    const [errorMessage, setErrorMessage] = useState("");

    const updateScenes = () => {
        tauri.invoke("get_scenes").then((res) => {
            const resDatas : Scene[] = JSON.parse(JSON.stringify(res));
            setScenes(resDatas);
        })
        .catch((err) => {
            setIsMessageDialogOpen(true);
            setErrorMessage(err);
        });
    }

    // インスタンス化されたときに一度だけログを吐く
    useEffect(() => {
        updateScenes();
    }, []);


    return (
        // シーンデータを画面内に並べる
        <div className="grid grid-cols-4 gap-4 w-screen py-5">
            {/* ExecuteSceneButtonを並べる */}
            {scenes.map((scene, index) => (
                <ExecuteSceneButton key={index} data={scene}/>
            ))}

            {/* リロードボタンを設ける */}
            <div className="flex justify-center items-center">
                <button className="col-span-4 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" onClick={updateScenes}>Reload</button>
            </div>

            {/* ダイアログ */}
            <MessageDialog
                isOpen={isMessageDialogOpen}
                onClose={() => setIsMessageDialogOpen(false)}
                errorMessage={errorMessage}
            />
        </div>
    );
}

export default SceneTab;