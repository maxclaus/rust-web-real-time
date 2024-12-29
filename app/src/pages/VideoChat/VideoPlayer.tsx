import { useContext } from "react";
import { SocketContext, SocketContextType } from "../../SocketContext";
import "./VideoPlayer.css";

const VideoPlayer = () => {
  const { name, callAccepted, myVideo, userVideo, callEnded, call } =
    useContext(SocketContext) as SocketContextType;

  return (
    <div className="videoplayer-container">
      {
        <div>
          <h5>{name || "Name"}</h5>
          <video playsInline muted ref={myVideo} autoPlay width="600" />
        </div>
      }
      {callAccepted && !callEnded && (
        <div>
          <h5>{call?.name || "Name"}</h5>
          <video playsInline ref={userVideo} autoPlay width="600" />
        </div>
      )}
    </div>
  );
};
export default VideoPlayer;
