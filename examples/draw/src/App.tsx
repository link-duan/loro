import { useEffect, useRef, useState } from 'react'
import {Loro} from 'loro-crdt';
import {throttle} from "throttle-debounce";
import './App.css'

function App() {
  const ref = useRef<HTMLCanvasElement>(null)
  const [size, setSize] = useState(0);
  const [time, setTime] = useState(0);
  useEffect(() => {
    if(!ref.current) {
      return;
    }

    const doc = new Loro();
    const map = doc.getMap("movement");
    const listener = throttle(100,   (e: MouseEvent) => {
      stop();
      setTime(time => time + 100);
      console.log(e.pageX, e.pageY);
      map.set("x", e.pageX);
      map.set("y", e.pageY);
      const size = doc.exportFrom().length;
      setSize(size);
    });

    ref.current.addEventListener("mousemove", listener);
    () => {
      ref.current?.removeEventListener("mousemove", listener);
    }
  }, [])

  return (
    <>
      <div>
        Time {time/1000}s
      </div>
      <div>
        The size of the doc {size} bytes
      </div>
      <canvas id="canvas" ref={ref} width={500} height={500} style={{background: "red"}}/>
    </>
  )
}

export default App
