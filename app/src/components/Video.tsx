import { useState } from 'react'

function Video() {
  const [count, setCount] = useState(0)

  return (
    <video id="gum-local" autoplay playsinline></video>
  );
}

export default Video
