import 'sakura.css';
import "./App.css";
import { Calendar } from "./component";
import { useEffect, useState } from 'react';
import { getInvoke } from './utility';

const invoke = getInvoke();

function App() {
  return (
    <>
      <Calendar />
    </>
  );
}

export default App;
