import { StrictMode } from "react";
import "@fontsource-variable/geist";
import { createRoot } from "react-dom/client";
import App from "./App";

const root = document.getElementById("root");

if (root === null) {
  throw new Error("Root element was not found");
}

createRoot(root).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
