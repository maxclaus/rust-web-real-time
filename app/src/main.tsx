import "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import Home from "./pages/Home/Home.tsx";
import EventStream from "./pages/EventStream/EventStream.tsx";
import VideoChat from "./pages/VideoChat/VideoChat.tsx";
import { BrowserRouter, Routes, Route } from "react-router";
import { SocketContextProvider } from "./SocketContext";

createRoot(document.getElementById("root")!).render(
  <SocketContextProvider>
    <BrowserRouter>
      <Routes>
        <Route element={<App />}>
          <Route index element={<Home />} />
          <Route path="eventstream" element={<EventStream />} />
          <Route path="videochat" element={<VideoChat />} />
        </Route>
      </Routes>
    </BrowserRouter>
  </SocketContextProvider>,
);
