// 押下するとtauriの関数を読み出すボタン
// 関数へは引数としてボタンの番号を渡す

import { invoke } from "@tauri-apps/api/tauri";
import React, { useState } from "react";

interface Props {
  num: number;
}

export const ActionButton: React.FC<Props> = ({ num }) => {
  const executeMsg : string = "execute" + num;

  async function execute() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log(await invoke("execute", { num }));
  }

  return (
    <div>
        {/* 押下時にexecuteを呼び出すボタン */}
        <button onClick={execute}>{executeMsg}</button>
    </div>
  );
};