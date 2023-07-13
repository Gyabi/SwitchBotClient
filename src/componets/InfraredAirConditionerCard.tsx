import { tauri } from "@tauri-apps/api";
import { FC } from "react";
import { useState } from "react";
import { InfraredRemote } from "../data/InfraredRemote";
import { AirCondFanSettingValue, AirCondModeSettingValue } from "../data/InfraredAirConditioner";

interface InfaredAirConditionerCardProps {
    data: InfraredRemote
}

const InfaredAirConditionerCard: FC<InfaredAirConditionerCardProps> = ({ data }) => {
    const [enable, setEnable] = useState(false);
    
    const onClickSwitchButton = (status: boolean) => {
        tauri.invoke("execute_infrated_remote_enable", {device_id: data.deviceId, enable: status})
        .catch((err) => {
            console.error(err);
        });
        setEnable(status);
    }

    // 温度変化用の変数
    const [temperature, setTemperature] = useState(25);
    // モード変化用の変数
    const [mode, setMode] = useState(AirCondModeSettingValue.AUTO);
    // 風量変化用の変数
    const [fan, setFan] = useState(AirCondFanSettingValue.AUTO);
    // 実行用の関数
    const onClickExecuteButton = () => {
        tauri.invoke("execute_infrated_airconfitioner_command", {device_id: data.deviceId, temperature: temperature, mode: mode, fan_speed: fan, power_state: true})
        .catch((err) => {
            console.error(err);
        });
    }
    const onChangeMode = (e: any) => {
        switch (e.target.value) {
            case "1" :
                setMode(AirCondModeSettingValue.AUTO);
                break;
            case "2" :
                setMode(AirCondModeSettingValue.COOL);
                break;
            case "3" :
                setMode(AirCondModeSettingValue.DRY);
                break;
            case "4" :
                setMode(AirCondModeSettingValue.FAN);
                break;
            case "5" :
                setMode(AirCondModeSettingValue.HEAT);
                break;
            default:
                console.error("mode change error");    
                break;
        }
    }
    const onChangeFan = (e: any) => {
        switch (e.target.value) {
            case "1" :
                setFan(AirCondFanSettingValue.AUTO);
                break;
            case "2" :
                setFan(AirCondFanSettingValue.LOW);
                break;
            case "3" :
                setFan(AirCondFanSettingValue.MEDIUM);
                break;
            case "4" :
                setFan(AirCondFanSettingValue.HIGH);
                break;
            default:
                console.error("fan change error");    
                break;
        }
    }
    
    return (
        // tailwindcssでcardを作る。各種情報を表示し、enabledを切り替える為のボタンを設ける。
        <div className="flex flex-col bg-gray shadow-md rounded px-8 pt-6 pb-8 mb-4">
            <p className="text-xl font-bold mb-4">{data.deviceName}</p>
            <p className="text-gray-700 text-base">status: {enable ? "ON" : "OFF"}</p>
            <div className="flex justify-between">
                <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onClick={()=>{onClickSwitchButton(false)}}>
                    OFF
                </button>
                <button className="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" onClick={()=>{onClickSwitchButton(true)}}>
                    ON
                </button>
            </div>

            {/* 温度、AirCondModeSettingValue、AirCondFanSettingValueを設定して実行する入力エリアを作成 */}
            <div className="flex flex-col">
                <div className="flex mt-10 mb-2 justify-start">
                    <p className="text-gray-700 text-xl font-bold">Temp:</p>
                    <input value={temperature} onChange={(e) => setTemperature(parseInt(e.target.value))} data-te-input-init className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="temperature" type="number" placeholder="temperature" step="1" min="0" max="30" />
                </div>
                <div className="flex my-2 justify-start">
                    <p className="text-gray-700 text-xl font-bold">Mode:</p>
                    <select value={mode} onChange={(e) => onChangeMode(e)} data-te-select-init className="w-full">
                        <option value={AirCondModeSettingValue.AUTO}>AUTO</option>
                        <option value={AirCondModeSettingValue.COOL}>COOL</option>
                        <option value={AirCondModeSettingValue.DRY}>DRY</option>
                        <option value={AirCondModeSettingValue.FAN}>FAN</option>
                        <option value={AirCondModeSettingValue.HEAT}>HEAT</option>
                    </select>
                </div>
                <div className="flex my-2 justify-start">
                    <p className="text-gray-700 text-xl font-bold">Fan:</p>
                    <select value={fan} onChange={(e) => onChangeFan(e)} data-te-select-init className="w-full">
                        <option value={AirCondFanSettingValue.AUTO}>AUTO</option>
                        <option value={AirCondFanSettingValue.LOW}>LOW</option>
                        <option value={AirCondFanSettingValue.MEDIUM}>MID</option>
                        <option value={AirCondFanSettingValue.HIGH}>HIGH</option>
                    </select>
                </div>

                <button data-te-button-init className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded my-2" onClick={onClickExecuteButton}>
                    Execute
                </button>
            </div>
            
        </div>
    );
};

export default InfaredAirConditionerCard;