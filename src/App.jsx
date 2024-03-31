import { Route, Routes, Navigate } from "react-router-dom";
import Layout from "./components/layout";
import Tasks from "./pages/tasks";
import Settings from "./pages/settings";
import { Command } from "@tauri-apps/api/shell";
import { useEffect } from "react";
import { resolveResource } from "@tauri-apps/api/path";

async function call() {
  const resourcePath = await resolveResource("resources/aria2.conf");
  console.log(888, resourcePath);
  const command = Command.sidecar("binaries/aria2c", [
    "--conf-path=" + resourcePath,
    "-D",
  ]);
  const output = await command.execute();
  console.log(output);
  console.log(output.stdout);
}

function App() {
  useEffect(() => {
    call();
  }, []);
  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<Navigate to="/tasks" />}></Route>
        <Route path="tasks" element={<Tasks />}></Route>
        <Route path="settings" element={<Settings />}></Route>
      </Route>
    </Routes>
  );
}

export default App;
