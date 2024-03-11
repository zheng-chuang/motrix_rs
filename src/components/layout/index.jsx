import { NavLink, Outlet } from "react-router-dom";
import cls from "classnames"
import classnames from "classnames/bind";

import LogoMini from "@/assets/logo-mini.svg?react"
import MenuTask from "@/assets/icons/menu-task.svg?react"
import MenuAdd from "@/assets/icons/menu-add.svg?react"
import MenuPreference from "@/assets/icons/menu-preference.svg?react"
import MenuAbout from "@/assets/icons/menu-about.svg?react"

import style from './index.module.less';

const cx = classnames.bind(style)

function Layout() {
    return <div className={cx('container-wrap')}>
        <div data-tauri-drag-region className={cx('drag-region')} />
        <div className={cx('container')}>
            <div className={cx('left')}>
                <div className={cx('logo')}>
                    <a target="_blank" href="https://motrix.app/">
                        <LogoMini />
                    </a>
                </div>
                <NavLink to="/tasks" style={{ marginTop: '24px' }}>
                    <div className={cx('icon-wrap')}>
                        <MenuTask className={cx('icon')} />
                    </div>
                </NavLink>
                <div className={cx('icon-wrap')} style={{ marginTop: '24px' }}>
                    <MenuAdd className={cx('icon')} />
                </div>
                <div style={{ flex: 1 }}></div>
                <NavLink to="/settings">
                    <div className={cx('icon-wrap')}>
                        <MenuPreference className={cx('icon')} />
                    </div>
                </NavLink>

                <div className={cls(cx('icon-wrap'), cx('icon-wrap-last'))} style={{ marginTop: '24px' }}>
                    <MenuAbout className={cx('icon')} />
                </div>
            </div>
            <div className={cx('right')}>
                <Outlet />
            </div>
        </div>
    </div>
}
export default Layout;