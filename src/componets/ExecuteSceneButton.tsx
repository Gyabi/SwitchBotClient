// 押下するとtauriの関数を読み出すボタン
// 関数へは引数としてボタンの番号を渡す

import { invoke } from "@tauri-apps/api/tauri";
import React, { useState } from "react";
import { Scene } from "../data/Scene";

interface Props {
  data: Scene;
}

export const ExecuteSceneButton: React.FC<Props> = ({data}) => {
  const [loading, setLoading] = useState(false);
  const executeMsg : string = data.scene_name;

  async function execute() {
    setLoading(true);
    // 対応するシーンを実行する。await中はカーソルを無効化する。失敗した場合はログをエラーログを残す
    await invoke("execute_scene", {scene_id: data.scene_id}).catch((e) => {
      setLoading(false);
      console.error(e);
    });
    setLoading(false);
  }

  return (
    <div>
        {/* 押下時にexecuteを呼び出すボタン */}
        {loading && <div>Loading...</div>}
        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onClick={execute}>{executeMsg}</button>
    </div>
  );
};