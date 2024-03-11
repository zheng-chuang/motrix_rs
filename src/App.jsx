import { Route, Routes, Navigate } from "react-router-dom";
import Layout from "./components/layout";
import Tasks from "./pages/tasks";
import Settings from "./pages/settings";

function App() {
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
