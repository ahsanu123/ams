// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";
import 'sakura.css';
import "./App.css";
import { Calendar } from "./component";
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const [greetMsg, setGreetMsg] = useState("");

  async function greet() {
    const message = await invoke("hello", { name: "another world" }) as string;
    setGreetMsg(message);
  }

  useEffect(() => {
    greet()
  }, []);

  return (
    <>
      <h2>{greetMsg}</h2>
      <Calendar />
    </>
  );
}

export default App;
