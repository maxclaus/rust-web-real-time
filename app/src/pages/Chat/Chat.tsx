import { useState, useCallback, useEffect } from "react";
import "react";
import "./Chat.css";

interface VideoChatProps {
  serverURL: string;
}

interface Message {
  from_user_id: string;
  from_user_name: string;
  message: string;
}

function Chat({ serverURL }: VideoChatProps) {
  const [name, setName] = useState("");
  const [me, setMe] = useState("");
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [messages, setMessages] = useState<Array<Message>>([]);
  const [message, setMessage] = useState("");

  useEffect(() => {
    // NOTE: use a hard coded id for this demos, but in real life
    // it should be handled by provided by the backend.
    let roomId = "4de940c6-e46a-4509-856e-12811fd6cbb1";
    const ws = new WebSocket(`${serverURL}/api/ws/rooms/${roomId}`);

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
        case "Message": {
          setMessages((messages) => [...messages, msg.data]);
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
  }, [ws]);

  const sendMessage = useCallback(() => {
    ws?.send(
      JSON.stringify({
        type: "Message",
        data: {
          from_user_id: me,
          from_user_name: name,
          message,
        },
      }),
    );
    setMessage("");
  }, [ws, me, name, message]);

  return (
    <div>
      <h2>WebSocket - Chat</h2>
      <p>
        This is an example of a text chat using{" "}
        <a
          href="https://developer.mozilla.org/en-US/docs/Web/API/WebSocket"
          target="_blank"
        >
          Web Socket
        </a>
        {" for real time data communication."}
      </p>
      <div className="messages">
        {messages.length > 0
          ? messages.map((msg, index) => (
              <div key={index}>
                <strong>{msg.from_user_name}</strong>: {msg.message}
              </div>
            ))
          : "-- no messages yet --"}
      </div>
      <div className="input-fields">
        <label>Name</label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          width="100%"
        />
        <label>Message</label>
        <input
          type="text"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          width="100%"
        />
        <input type="button" value="Send" onClick={sendMessage} />
      </div>
    </div>
  );
}

export default Chat;
