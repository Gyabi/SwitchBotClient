// シーン開始時に赤外線デバイスの一覧を表示（現状はライトとエアコンだけ表示する）
// それぞれに操作ボタンを配置してボタン押下で操作可能とする
// 暫定的にエアコンもライト用のカードで代用する
import { useEffect, useState } from "react";
import { tauri } from "@tauri-apps/api";
import InfraredLightCard from "../InfraredLightCard";
import MessageDialog from "../MessageDialog";
import { InfraredRemote } from "../../data/InfraredRemote";

const InfraredDeviceTab = () => {
    const [infraredDevices, setInfraredDevices] = useState<InfraredRemote[]>([]);
    // ダイアログ制御用変数
    const [isMessageDialogOpen, setIsMessageDialogOpen] = useState(false);
    const [errorMessage, setErrorMessage] = useState("");
    
    const updateInfaredDevices = () => {
        tauri.invoke("get_infrared_remotes").then((res) => {
            const resDatas : InfraredRemote[] = JSON.parse(JSON.stringify(res));
            setInfraredDevices(resDatas);
        })
        .catch((err) => {
            setIsMessageDialogOpen(true);
            setErrorMessage(err);
        });
    }

    // インスタンス化されたときに一度だけログを吐く
    useEffect(() => {
        updateInfaredDevices();
    }, []);


    return (
        // 赤外線デバイスデータを画面内に並べる
        <div className="grid grid-cols-4 gap-4 w-screen">
            {/* InfraredLightCardを並べる */}
            {infraredDevices.map((infraredDevice, index) => (
                <InfraredLightCard key={index} data={infraredDevice}/>
            ))}

            {/* リロードボタンを設ける */}
            <div className="flex justify-center items-center">
                <button className="col-span-4 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" onClick={updateInfaredDevices}>Reload</button>
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

export default InfraredDeviceTab;