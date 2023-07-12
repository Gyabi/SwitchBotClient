import { tauri } from "@tauri-apps/api";
import { FC } from "react";
import { useState } from "react";
import { InfraredRemote } from "../data/InfraredRemote";

interface InfraredLightCardProps {
    data: InfraredRemote
}

const InfraredLightCard: FC<InfraredLightCardProps> = ({ data }) => {
    const [enable, setEnable] = useState(false);
    
    const onClickSwitchButton = () => {
        tauri.invoke("execute_infrated_remote_enable", {device_id: data.deviceId, enable: !enable})
        .catch((err) => {
            console.error(err);
        });
        setEnable(!enable);
    }
    
    return (
        // tailwindcssでcardを作る。各種情報を表示し、enabledを切り替える為のボタンを設ける。
        <div className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
            <p className="text-xl font-bold mb-4">{data.deviceName}</p>
            <p className="text-gray-700 text-base">deviceID: {data.deviceId}</p>
            <p className="text-gray-700 text-base">enabled: {enable ? "true" : "false"}</p>
            <div className="flex justify-end">
                <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onClick={onClickSwitchButton}>
                    Switch
                </button>
            </div>
        </div>
    );
};

export default InfraredLightCard;