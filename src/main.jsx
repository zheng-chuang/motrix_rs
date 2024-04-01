import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
// sidecar
import { Command } from "@tauri-apps/api/shell";
import { resolveResource } from "@tauri-apps/api/path";
import { once } from "@tauri-apps/api/event";
import App from "./app";

import "normalize.css/normalize.css";
import "./style.less";

// async function startAria2c() {
//   const resourcePath = await resolveResource("resources/aria2.conf");
//   console.log(888, resourcePath);
//   const command = Command.sidecar("binaries/aria2c", [
//     "--conf-path=" + resourcePath,
//   ]);
//   const child = await command.spawn();
//   console.log(child);
//   command.execute();
//   once("tauri://close-requested", () => {
//     child.kill();
//   });
// }
// startAria2c();

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>
);
