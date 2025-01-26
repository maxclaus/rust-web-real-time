import "react";
import { Outlet, Link } from "react-router";
import "./App.css";

function App() {
  const Divider = () => <span>|</span>;
  return (
    <div>
      <h1>Web Real Time Apps</h1>
      <div className="menu">
        <Link to="/">Home</Link>
        <Divider />
        <Link to="/eventstream">Event Stream</Link>
        <Divider />
        <Link to="/chat">Chat</Link>
        <Divider />
        <Link to="/videochat">Video Chat</Link>
      </div>
      <Outlet />
    </div>
  );
}

export default App;
