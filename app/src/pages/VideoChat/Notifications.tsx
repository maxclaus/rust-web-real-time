import { useContext } from "react";
import { SocketContext } from "../../SocketContext";
import "./Notifications.css";

const Notifications = () => {
  const { answerCall, call, callAccepted, leaveCall } =
    useContext(SocketContext);

  return (
    <>
      {call.isReceivingCall && !callAccepted && (
        <>
          <div className="notifications-modal-overlay"></div>
          <div className="notifications-modal">
            <div className="notifications-modal-content">
              <h2> {call.name || "Unknown caller"} is calling </h2>
            </div>
            <div className="notifications-modal-buttons">
              <button
                className="notifications-modal-accept"
                onClick={answerCall}
              >
                Answer Call
              </button>
              <button
                className="notifications-modal-reject"
                onClick={leaveCall}
              >
                Reject Call
              </button>
            </div>
          </div>
        </>
      )}
    </>
  );
};
export default Notifications;
