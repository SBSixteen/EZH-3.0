import { useState } from "react";
import React from "react";
import "./App.css";
import "./style.css"
import Sidebar from "./Sidebar.jsx";
import "./Home.css";

function Datasetform() {
  
const [name, setName] = useState("");
const [deadline, setDeadline] = useState("");
const [file, setFile] = useState({
    resumeParsing:"",
    importDataset:"",
});

const handleFileChange = (e) => {
    if (e.target.files) {
      setFile(e.target.files[0]);
    }
  };

  
return (

  <div className="container-new">
      <Sidebar/>
      <div className="others">
      <div className="container">

<div className="row">
  <a href="" target="_blank">
    <img src="/Logo_Ezhire.svg" className="logo tauri" alt="Tauri logo" />
  </a>
</div>

<h1>Dataset</h1>

<div className="column">
  <form
  onSubmit={(e) =>{

    e.preventDefault();
  

  }}>

  <input
      className="default_gap"
      id="name-input"
      onChange={(e) => {
        setName(e.currentTarget.value);
        
      }}
      placeholder="Enter Dataset Name"
  />
  
  <br></br>
  
  <input 
      className="default_gap"
      type = "datetime-local"
      id="deadline-input"
      onChange={(e) => {
        setDeadline(e.currentTarget.value);
      }}
    />

<br></br>

<input
    type = "file" 
    value={file.resumeParsing}
    onChange={(e) =>
      setFile({ ...file, resumeParsing: e.target.value })
    }
/>

<br></br>

<input
    type = "file"
    value={file.importDataset}
    onChange={(e) =>
      setFile({ ...file, importDataset: e.target.value })
     }
/>
  <br></br>
  
    
  <button className="default_m_right" type="submit" onClick={
    ()=>{}
  }> Submit </button>

  </form>
</div>
</div>
      </div>
  </div>
  
  
);


}

  export default Datasetform;