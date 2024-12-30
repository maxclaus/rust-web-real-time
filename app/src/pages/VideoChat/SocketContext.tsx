import {
  createContext,
  useState,
  useRef,
  useEffect,
  useCallback,
  MutableRefObject,
} from "react";
import Peer from "simple-peer";

type CallData = {
  isReceivingCall: boolean;
  from: string;
  name: string;
  signal: string;
};

export type SocketContextType = {
  call: CallData | undefined;
  callAccepted: boolean;
  myVideo: MutableRefObject<HTMLVideoElement | null>;
  userVideo: MutableRefObject<HTMLVideoElement | null>;
  stream: MediaStream | undefined;
  name: string;
  setName: (name: string) => void;
  callEnded: boolean;
  me: string;
  callUser: (id: string) => void;
  leaveCall: () => void;
  answerCall: () => void;
};

const SocketContext = createContext<SocketContextType | null>(null);

type SocketContextProviderProps = {
  serverURL: string;
  children: React.ReactNode;
};

function SocketContextProvider({
  children,
  serverURL,
}: SocketContextProviderProps) {
  const [callAccepted, setCallAccepted] = useState(false);
  const [callEnded, setCallEnded] = useState(false);
  const [stream, setStream] = useState<MediaStream>();
  const [name, setName] = useState("");
  const [call, setCall] = useState<CallData | undefined>();
  const [me, setMe] = useState("");
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [callerPeer, setCallerPeer] = useState<Peer.Instance | null>(null);
  const myVideo = useRef<HTMLVideoElement>(null);
  const userVideo = useRef<HTMLVideoElement>(null);
  const connectionRef = useRef<Peer.Instance>();
  console.log("****SocketContextProvider video", myVideo.current);
  console.log("****SocketContextProvider stream", stream);

  useEffect(() => {
    let roomId = "41feb23b-7882-4754-a18a-4fbdaf0bcd77";
    const ws = new WebSocket(`${serverURL}/api/ws/videochat/${roomId}`);

    const openHandler = (event: Event) => {
      console.log("***Connection opened!", event);
    };

    ws.addEventListener("open", openHandler);

    setWs(ws);

    return () => {
      ws.removeEventListener("open", openHandler);
    };
  }, []);

  useEffect(() => {
    if (!ws) {
      // not ready yet
      return;
    }

    const messageHandler = (event: MessageEvent) => {
      console.log("***Message from server ", event.data);
      const msg = JSON.parse(event.data);
      switch (msg.type) {
        case "Me": {
          setMe(msg.data.id);
          break;
        }
        case "CallUser": {
          setCall({
            isReceivingCall: true,
            from: msg.data.called_by_user_id,
            name: msg.data.called_by_name_name,
            signal: JSON.parse(msg.data.signal),
          });
          break;
        }
        case "CallAccepted": {
          setCallAccepted(true);
          callerPeer?.signal(JSON.parse(msg.data.signal));
          break;
        }
        default: {
          throw new Error("Unssuported message type");
        }
      }
    };

    // Listen for messages
    ws.addEventListener("message", messageHandler);

    return () => {
      ws.removeEventListener("message", messageHandler);
    };
  }, [ws, callerPeer]);

  useEffect(() => {
    console.log("****getUserMedia loading video", myVideo.current);
    console.log("****getUserMedia loading stream", stream);
    navigator.mediaDevices
      .getUserMedia({ video: true, audio: true })
      .then((currentStream) => {
        setStream(currentStream);
        if (myVideo.current) {
          myVideo.current.srcObject = currentStream;
        }
        console.log("****getUserMedia loaded video", myVideo.current);
        console.log("****getUserMedia loaded stream", currentStream);
      });

    // socket.on('me', (id) => setMe(id));
    // socket.on('callUser', ({ from, name: callerName, signal }) => {
    //     setCall({ isReceivingCall: true, from, name: callerName, signal });
    // });
  }, [myVideo.current]);

  const answerCall = useCallback(() => {
    if (!stream || !call) {
      return;
    }

    setCallAccepted(true);
    const peer = new Peer({ initiator: false, trickle: false, stream });
    console.log("***answerCall: peer", peer);
    peer.on("signal", (data) => {
      console.log("***answerCall: Peer on signal", data);
      ws?.send(
        JSON.stringify({
          type: "AnswerCall",
          data: {
            signal: JSON.stringify(data),
            answer_to_user_id: call.from,
          },
        }),
      );
    });
    peer.on("stream", (currentStream) => {
      console.log("***answerCall: Peer on stream");
      if (userVideo.current) {
        userVideo.current.srcObject = currentStream;
      }
    });
    peer.signal(call.signal);
    connectionRef.current = peer;
  }, [stream, call]);

  const callUser = useCallback(
    (id: string) => {
      const peer = new Peer({ initiator: true, trickle: false, stream });
      setCallerPeer(peer);
      console.log("***callUser: peer", peer);
      peer.on("signal", (data) => {
        console.log("***callUser: Peer on signal", data);
        ws?.send(
          JSON.stringify({
            type: "CallUser",
            data: {
              calling_to_user_id: id,
              called_by_user_id: me,
              called_by_user_name: name,
              signal: JSON.stringify(data),
            },
          }),
        );
      });
      peer.on("stream", (currentStream) => {
        console.log("***callUser: Peer on streeam");
        if (userVideo.current) {
          userVideo.current.srcObject = currentStream;
        }
      });
      // socket.on('callAccepted', (signal) => {
      //     setCallAccepted(true);
      //     peer.signal(signal);
      // });
      connectionRef.current = peer;
    },
    [stream, me, name],
  );

  const leaveCall = useCallback(() => {
    setCallEnded(true);
    if (connectionRef.current) {
      connectionRef.current.destroy();
    }
    window.location.reload();
  }, []);

  return (
    <SocketContext.Provider
      value={{
        call,
        callAccepted,
        myVideo,
        userVideo,
        stream,
        name,
        setName,
        callEnded,
        me,
        callUser,
        leaveCall,
        answerCall,
      }}
    >
      {children}
    </SocketContext.Provider>
  );
}
export { SocketContextProvider, SocketContext };
