import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");


  async function StartServer() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("start_http_server", { name }));
  }


  async function StopServer() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("stop_http_server", { name }));
  }


  return (
    <div className="container">
   
    <button 
      onClick={StartServer}
      className="btn btn-primary"
      
    
    >
      start https server
    </button>

    <button 
      onClick={StopServer}
      className="btn btn-primary"

    >
      stop https server
    </button>
    <p>{greetMsg}</p>
    </div>
  );
}

export default App;
