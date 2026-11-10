// 根组件：目前只是一个占位页，确认 React + antd + Tauri 能跑起来
// 第一版会在这里换成"素材库列表"页面
import { Layout, Result } from "antd";

export default function App() {
  return (
    <Layout className="app-layout">
      <Layout.Content className="app-content">
        <Result status="info" title="VRAMS v2" subTitle="React + Ant Design 骨架已就绪，第一版开发中" />
      </Layout.Content>
    </Layout>
  );
}
