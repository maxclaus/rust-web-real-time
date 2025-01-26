# rust web real time

Demo app running on https://rust-web-real-time.onrender.com (it may take a few seconds for the initial loading).

Article about this project was posted on https://maxclaus.xyz/blog/web-real-time-apps/.

Project demo testing out the implementation of web real time apps using Rust for the backend and React for the frontend.

It contains samples for:

- [Server-sent events (SSE)](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events)
- [WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
- [WebRTC](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API)

## Running it in development

In one shell window run this command below to start the backend server:

```
cargo run
```

In another shell window run this command below to start the frontend app:

```
cd app
npm run dev
```

Then open the browser at address http://localhost:5173/.

## References

- https://webrtc.github.io/samples/
- https://medium.com/@Jayseabee/rust-react-part-i-3a33c3da9ca0
- https://cloudinary.com/blog/guest_post/stream-videos-with-webrtc-api-and-react
- https://github.com/antholeole/actix-sockets
- https://github.com/actix/examples/blob/master/websockets/echo-actorless
- https://github.com/agmcleod/questions-app-rust-actix (used as example for ws messages with different types)
- https://agmprojects.com/blog/building-a-rest-and-web-socket-api-with-actix.html
- https://www.baeldung.com/webrtc
- https://web.dev/articles/webrtc-infrastructure
- https://codelabs.developers.google.com/codelabs/webrtc-web
- https://medium.com/@fengliu_367/getting-started-with-webrtc-a-practical-guide-with-example-code-b0f60efdd0a7
