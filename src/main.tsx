// 前端入口：把根组件 <App /> 挂载到 index.html 的 #root 上
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App as AntdApp, ConfigProvider } from "antd";
import zhCN from "antd/locale/zh_CN";

import App from "@/App";
import "@/styles/global.css";

createRoot(document.getElementById("root")!).render(
  // StrictMode：开发模式下会故意把组件渲染两次，帮你提前发现副作用写错的地方
  <StrictMode>
    {/* ConfigProvider：antd 的全局配置，这里设置中文语言包 */}
    <ConfigProvider locale={zhCN}>
      {/* AntdApp：让 message / modal 等弹窗能拿到上面的全局配置 */}
      <AntdApp>
        <App />
      </AntdApp>
    </ConfigProvider>
  </StrictMode>,
);
