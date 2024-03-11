import { Outlet } from "react-router-dom";
import Logo from "../logo";
import LogoMini from "../logo-mini";

import classnames from "classnames/bind";
import cls from "classnames"

import style from './index.module.less';

const cx = classnames.bind(style)

function Layout() {
    return <div className={cx('container-wrap')}>
        <div data-tauri-drag-region className={cx('drag-region')} />
        <div className={cx('container')}>
            <div className={cx('left')}>
                <div className={cx('logo')}><LogoMini /></div>
            </div>
            <div className={cx('right')}>
                <Outlet />
            </div>
        </div>
    </div>
}
export default Layout;