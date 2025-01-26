import "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import Home from "./pages/Home/Home.tsx";
import EventStream from "./pages/EventStream/EventStream.tsx";
import VideoChat from "./pages/VideoChat/VideoChat.tsx";
import Chat from "./pages/Chat/Chat.tsx";
import { BrowserRouter, Routes, Route } from "react-router";

declare global {
  interface Window {
    appServerURL: string;
  }
}

const serverURL = import.meta.env.PROD
  ? `${window.location.protocol}//${window.location.host}`
  : "http://localhost:8080";
console.log("Using serverURL", serverURL, import.meta.env.BASE_URL);

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route element={<App />}>
        <Route index element={<Home />} />
        <Route
          path="eventstream"
          element={<EventStream serverURL={serverURL} />}
        />
        <Route path="chat" element={<Chat serverURL={serverURL} />} />
        <Route path="videochat" element={<VideoChat serverURL={serverURL} />} />
      </Route>
    </Routes>
  </BrowserRouter>,
);
