import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import divide from "./utils/image";
interface Payload {
  state: number[],
}
const stateMap = (state: number[]) => {
  let tmp: string[] = new Array(N * N);
  for (let i = 0; i < 9; i++) {

    for (let j = 0; j < 9; j++) {
      if (state[i] == MAP[j][0]) {
        tmp[i] = MAP[j][1]
        break
      }
    }
  }
  return tmp;
}
const N = 3;

const TARGET = await divide()
const target: number[] = [1, 2, 3, 8, 0, 4, 7, 6, 5]
const init: number[] = [1, 3, 4, 8, 6, 2, 7, 5, 0]
const MAP: [number, string][] = TARGET.map((img, i) => [target[i], img])
const initImg = stateMap(init)
function Chess() {
  const [imgs, setImgs] = useState<string[]>(initImg)
  const [state, setState] = useState<number[]>(init)
  const [steps, setSteps] = useState<number>(0)
  useEffect(() => {

    console.log('inside')
    const unlisten: Promise<UnlistenFn> = listen('state', (event) => {
      sleep(0).then(() => {
        let { state }: Payload = event.payload as Payload
        console.log('state: ' + state)
        stateTransform(state)
      })
    });
    return () => {
      console.log('return')
      unlisten.then(f => f())
    }
  }, [])


  function sleep(time: number) {
    return new Promise((resolve) => setTimeout(resolve, time));
  }
  const shuffle = (state: number[]) => {
    state.sort(() => Math.random() - 0.5)
  }
  async function test() {
    await invoke('reset')
    invoke('a_display', { init, target })
    invoke('count_steps', { init, target }).then((n: any) => { setSteps(n) })

  }

  const reset = async () => {
    await invoke('reset')
    setSteps(0)
    stateTransform(init)
  }
  const stateTransform = (state: number[]) => {
    setState(state)
    setImgs(stateMap(state))
  }
  const err = (e: any) => {
    console.log('error image')
  }
  const load = (e: any) => {
    console.log('load image')
  }
  return (
    <div className="App" style={{ height: "600px", width: "100%" }}>
      <div style={{ display: "flex", justifyContent: "space-between" }}>
        <button onClick={test}>搜索</button>
        <button onClick={reset}>reset</button>
        <p></p>
      </div>
      <div style={{ display: "flex", justifyContent: "space-between" }}>
        <p>状态:  {state}</p>
        <p>目标:  {target}</p>
        <h3>总次数:  {steps}</h3>
      </div>
      <div className='grid' style={{}}>
        {imgs.map((b64, index) => {
          return <img src={b64} alt="cell" key={index} onLoad={load} />
        })}
      </div>
    </div>
  );
}



function App() {


  return (
    <div className="App">
      <Chess />
    </div>
  );
}

export default App;
