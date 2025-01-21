// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";
import 'sakura.css';
import "./App.css";
import { Calendar } from "./component";

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");
  //
  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <>
      <h2>jjj</h2>
      <p>
        check
      </p>
      <Calendar />
    </>
  );
}

export default App;
