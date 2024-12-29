import "react";
import "./VideoChat.css";
import VideoPlayer from "./VideoPlayer.tsx";
import CallOptions from "./CallOptions.tsx";
import Notifications from "./Notifications.tsx";
import { SocketContextProvider } from "./SocketContext";

function VideoChat() {
  return (
    <SocketContextProvider>
      <h2>Video Chat</h2>
      <p>
        This is an example of a video chat using these technologies{" "}
        <a
          href="https://developer.mozilla.org/en-US/docs/Web/API/WebSocket"
          target="_blank"
        >
          Web Socket
        </a>{" "}
        ....
      </p>
      <VideoPlayer />
      <CallOptions />
      <Notifications />
    </SocketContextProvider>
  );
}

export default VideoChat;
