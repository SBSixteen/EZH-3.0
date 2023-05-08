import React, { useState, useRef, useEffect } from "react";
import "./App.css";
import "./style.css";
import Sidebar from "./Sidebar.jsx";
import "./Home.css";
import "./Datasetform.css";
import * as FileSaver from "file-saver";
import { Publish } from "@material-ui/icons";


function Datasetform() {
  const [name, setName] = useState("");
  const [deadline, setDeadline] = useState("");
  const [selectedFolder, setSelectedFolder] = useState([]);
  const [isFileUploaded, setIsFileUploaded] = useState(false);
  const [base64String, setBase64String] = useState("");
  const [fileBytes, setFileBytes] = useState([]);
  const base64StringRef = useRef("");

  useEffect(() => {
    base64StringRef.current = base64String;
  }, [base64String]);

  // function handleFileUpload(event) {
  //   if (isFileUploaded) {
  //     alert("You can only upload one file");
  //     return;
  //   }
  //   setIsFileUploaded(true);
  // }

  // function handleSubmit(event) {
  //   event.preventDefault();
  //   if (!isFileUploaded) {
  //     alert("Please upload a file");
  //     return;
  //   }

  //   // saveBase64ToFile(base64StringRef.current);
  // }

  function handleChange(event) {
    const file = event.target.files[0];
    const extension = file.name.split(".").pop().toLowerCase();

    if (extension === "zip") {
      const reader = new FileReader();
      reader.onload = function (event) {
        try {
          const bytes = new Uint8Array(event.target.result);
          const byteArr = Array.from(bytes);
          setFileBytes(byteArr);
          const binary = bytes.reduce(
            (acc, byte) => acc + String.fromCharCode(byte),
            ""
          );
          const base64 = btoa(binary);
          setBase64String(base64);
          setSelectedFolder(file.name);
        } catch (error) {
          console.error(error);
          alert("Error occurred while processing the zip file.");
        }
      };
      reader.readAsArrayBuffer(file);
    } else if (extension === "pdf" || extension === "docx") {
      const reader = new FileReader();
      reader.onload = function (event) {
        try {
          const base64 = btoa(event.target.result);
          setBase64String(base64);
          setSelectedFolder(file.name);
        } catch (error) {
          console.error(error);
          alert("Error occurred while processing the file.");
        }
      };
      reader.readAsBinaryString(file);
    } else {
      alert("Invalid file type. Please upload a .pdf, .docx, or .zip file.");
    }
  }

  function saveBase64ToFile(event) {
    event.preventDefault();
    const binaryString = base64String;
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    const file = new Blob([bytes], { type: "application/octet-stream" });
    FileSaver.saveAs(file, "my-file.txt");
  }
  
  /*return (
    <div className="container-new">
      <Sidebar />
      <div className="others">
        
          <div className="holder">
            <div className="h1datasetform">
              <h1>Dataset</h1>
            </div>
            <div className="column">
              <form className="datasetForm" onSubmit={handleSubmit}>
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
                  type="datetime-local"
                  id="deadline-input"
                  onChange={(e) => {
                    setDeadline(e.currentTarget.value);
                  }}
                />
                <br></br>
                <div className="datasetUpdateRight">
                <input type="file" onChange={handleChange} />
                <br></br>
                <p>base64: {base64String.slice(0, 30)}</p>
                </div>
                <br></br>
                <button
                  className="default_m_right"
                  type="submit"
                  onClick={saveBase64ToFile}
                >
                  Submit
                </button>
              </form>
            </div>
          </div>
        
      </div>
    </div>
  );*/

  
  return (
    <>
    <div className="container-new">
    <Sidebar />
      <div className="others">
          <div className="holder">
            <span className="datasetUpdateTitle">Create Dataset</span>
            <form className="datasetUpdateForm">
              <div className="datasetUpdateLeft">
                <div className="datasetUpdateItem">
                  <label>Dataset Name</label>
                  <input
                    type="text"
                    placeholder="Enter Dataset Name"
                    className="datasetUpdateInput" />
                </div>
                <div className="datasetUpdateItem">
                  <label>Deadline</label>
                  <input
                    type="datetime-local"
                    id="deadline-input"
                    className="datasetUpdateInput"
                    onChange={(e) => {
                      setDeadline(e.currentTarget.value);
                    }} 
                  />
                </div>
              </div>
              <div className="datasetUpdateRight">
                <div className="datasetUpdateUpload">
                  <label htmlFor="file" className="customButton">
                    <Publish className="datasetUpdateIcon" />
                    Upload Dataset
                  </label>
                  <input type="file" id ='file' onChange={handleChange} style={{ display: "none" }} />
                  <br></br>
                </div>
                <button
                  className="datasetUpdateButton"
                  type="submit"
                  onClick={saveBase64ToFile}
                >
                  Create
                </button>
              </div>
            </form>
          </div>
        </div>
      </div>
    </>
  )
}

export default Datasetform;
