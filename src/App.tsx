import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import divide from "./utils/image";
interface Payload {
  state: number[],
}
function App() {
  const [cells, setCells] = useState<string[]>([])
  const [map, setMap] = useState<Map<number, String>>(new Map())
  const n = 3;
  const target: number[] = [1, 2, 3, 4, 0, 5, 6, 7, 8]

  useEffect(() => {
    const unlisten: Promise<UnlistenFn> = listen('state', (event) => {
      let { state }: Payload = event.payload as Payload
      console.log('state: ' + state)
      cellTransform(state)
    });
    display()
    return () => { unlisten.then(f => f()) }
  }, [])

  const cellTransform = (state: number[]) => {
    let arr: string[] = Array(n * n)
    state.forEach(i => arr.push((map as Map<number, string>).get(i) as string))
    setCells(arr)
  }

  const display = async () => {
    let arr = await divide()
    setCells(arr)
    let m: Map<number, String> = new Map()
    for (let i = 0; i < arr.length; i++) {
      m.set(target[i], arr[i])
    }
    setMap(m)
  }

  const shuffle = (state: number[]) => {
    state.sort(() => Math.random() - 0.5)
  }
  async function test() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let state = [1, 2, 3, 4, 5, 6, 0, 7, 8]
    cellTransform(state)
    invoke('search', { state, target })
  }
  const reset = async () => {
    cellTransform(target)
    await invoke('reset')
  }
  return (
    <div className="App">
      <div className='grid' style={{}}>
        {cells.map((b64, index) => <img src={b64} alt="cell" key={index}></img>)}
      </div>
      <button onClick={test}>test</button>
      <button onClick={reset}>reset</button>
    </div>
  );
}

export default App;
