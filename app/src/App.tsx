import { useState } from "react";
import { Outlet, Link } from "react-router";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";

function App() {
  return (
    <div>
      <h1>Web Real Time Apps</h1>
      <div>
        <Link to="/">Home</Link> | <Link to="/eventstream">Event Stream</Link> |{" "}
        <Link to="/videochat">Video Chat</Link>
      </div>
      <Outlet />
    </div>
  );
}

export default App;
