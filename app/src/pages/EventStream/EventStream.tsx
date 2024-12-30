import { useState, useEffect } from "react";
import "./EventStream.css";

interface EventStreamProps {
  serverURL: string;
}

function EventStream({ serverURL }: EventStreamProps) {
  const [events, setEvents] = useState<Array<string>>([]);
  const [connected, setConnected] = useState<boolean>(true);

  useEffect(() => {
    // SSE (Server-Sent Events)
    let sse = new EventSource(`${serverURL}/api/eventstream`);
    sse.onopen = (e) => {
      console.log("EventStream open", e);
      setEvents([]);
      setConnected(true);
    };
    sse.onmessage = (e) => {
      console.log("EventStream event", events, e.data);
      setEvents((curr) => {
        return [...curr, e.data];
      });
    };
    sse.onerror = (err) => {
      // When an event stream is closed due
      console.log("EventStream error", err);
      setConnected(false);
      sse?.close();
    };

    return () => {
      sse?.close();
    };
  }, []);

  return (
    <>
      <h2>Event Stream</h2>
      <p>
        This is an example of{" "}
        <a
          href="https://developer.mozilla.org/en-US/docs/Web/API/EventSource"
          target="_blank"
        >
          Event Source
        </a>{" "}
        implementation.
      </p>
      <p>{connected ? "Connected: loading events" : "Disconnected"}.</p>
      {
        <p>
          {events.map((w) => (
            <span className="eventstream-word">{w + " "}</span>
          ))}
        </p>
      }
    </>
  );
}

export default EventStream;
