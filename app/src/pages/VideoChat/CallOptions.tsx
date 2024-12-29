import { useState, useContext } from "react";
import { CopyToClipboard } from "react-copy-to-clipboard";
import { BiClipboard, BiPhoneCall, BiPhoneOff } from "react-icons/bi";
import { SocketContext, SocketContextType } from "../../SocketContext";
import "./CallOptions.css";

const Options = () => {
  const { me, callAccepted, name, setName, callEnded, leaveCall, callUser } =
    useContext(SocketContext) as SocketContextType;
  const [idToCall, setIdToCall] = useState("");

  return (
    <div className="calloptions-container">
      <div>
        <h4> Account Info</h4>
        <div className="form-item">
          <label>Username</label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            width="100%"
          />
          <CopyToClipboard text={me}>
            <button>
              <BiClipboard />
              Copy ID
            </button>
          </CopyToClipboard>
        </div>
      </div>
      <div>
        <h4> Make a Call</h4>
        <div className="form-item">
          <label> User id to call </label>
          <input
            type="text"
            value={idToCall}
            onChange={(e) => setIdToCall(e.target.value)}
          />
          {callAccepted && !callEnded ? (
            <button onClick={leaveCall}>
              <BiPhoneOff />
              Hang up
            </button>
          ) : (
            <button onClick={() => callUser(idToCall)}>
              <BiPhoneCall />
              Call
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
export default Options;
