import React from "react";
import { StyleProvider  } from '@ant-design/cssinjs';
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <StyleProvider hashPriority="high">

    <App />
    </StyleProvider>
  </React.StrictMode>,
);
