import { useState } from "react";
import React from "react";
import "./App.css";
import "./style.css"
import Sidebar from "./Sidebar.jsx";
import "./Home.css";
import "./Datasetform.css"


function Datasetform() {
  const [name, setName] = useState("");
  const [deadline, setDeadline] = useState("");
  const [selectedFolder, setSelectedFolder] = useState([]);
  const [isFileUploaded, setIsFileUploaded] = useState(false);
  const [base64String, setBase64String] = useState("");

  function handleFileUpload(event) {
    if (isFileUploaded) {
      alert('You can only upload one file');
      return;
    }
    setIsFileUploaded(true);
  }

  function handleSubmit(event) {
    event.preventDefault();
    if (!isFileUploaded) {
      alert('Please upload a file');
      return;
    }
  }

  function handleChange(event) {
    const file = event.target.files[0];
    const reader = new FileReader();
    reader.onload = function(event) {
      const base64 = btoa(event.target.result);
      setBase64String(base64);
      setSelectedFolder(file.name);
    };
    reader.readAsBinaryString(file);
  }
  
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
          <div className="holder">
            <div className="h1datasetform">
              <h1 >Dataset</h1>
            </div> 
            <div className="column">
              <form onSubmit={handleSubmit}>
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
                onChange={handleChange}
              />
              <br></br>
              {selectedFolder && <p>You selected the file: {selectedFolder}</p>}
              {/* {base64String && <p>base64: {base64String}</p>} */}
              <br></br>
              <button 
                className="default_m_right" 
                type="submit" 
                onClick={()=>{}}> Submit 
              </button>

              </form>
            </div>
          </div>
        </div>  
      </div>
    </div>
  );
}

export default Datasetform;
