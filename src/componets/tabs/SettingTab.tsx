// tokenとsecretの設定を行うタブ
import { tauri } from "@tauri-apps/api";
import { useState } from "react";
import MessageDialog from "../MessageDialog";

// カード上記設定値を入力可能にして、ボタンで実行すると上書きするようにする
const SettingTab = () => {
    // ダイアログ制御用変数
    const [isMessageDialogOpen, setIsMessageDialogOpen] = useState(false);
    const [errorMessage, setErrorMessage] = useState("");

    // apiを使ってtokenとsecretを上書きする
    const updateTokenAndSecret = (token: string, secret: string) => {
        tauri.invoke("save_parameter", {parameter: {token: token, secret: secret}}).then((res) => {
            // 正常に登録したことをダイアログで通知
            setIsMessageDialogOpen(true);
            setErrorMessage("Token and Secret are updated.");
        })
        .catch((err) => {
            // ダイアログで正しいパスワードを設定するように促す
            setIsMessageDialogOpen(true);
            setErrorMessage(err);
        });
    }

    return (
        <div>
            <div className="grid grid-cols-2 gap-4 w-screen">
                <div className="col-span-2">
                    <div className="flex justify-center items-center">
                        <p className="text-2xl">Token</p>
                    </div>
                    <div className="flex justify-center items-center">
                        <input id="Token" className="w-1/2" type="text" placeholder="Token" />
                    </div>
                </div>
                <div className="col-span-2">
                    <div className="flex justify-center items-center">
                        <p className="text-2xl">Secret</p>
                    </div>
                    <div className="flex justify-center items-center">
                        <input id="Secret" className="w-1/2" type="text" placeholder="Secret" />
                    </div>
                </div>
                {/* 実行ボタン */}
                <div className="flex justify-center items-center">
                    {/* onClickでTokenとsecretを取得して、updateTokenAndSecretに渡す */}
                    <button className="col-span-4 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" onClick={() => updateTokenAndSecret((document.getElementById("Token") as HTMLInputElement).value, (document.getElementById("Secret") as HTMLInputElement).value)}>Save</button>
                </div>                    
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
export default SettingTab;
