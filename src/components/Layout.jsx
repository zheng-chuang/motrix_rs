import { Outlet } from "react-router-dom";

function Layout() {
    return <div>
        <div data-tauri-drag-region style={{ height: '30px', }}></div>
        <h1>Layout</h1>
        <Outlet />
    </div>
}
export default Layout;