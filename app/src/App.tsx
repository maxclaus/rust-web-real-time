import "react";
import { Outlet, Link } from "react-router";
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
