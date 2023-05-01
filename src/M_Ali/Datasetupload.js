import React,{useState} from 'react'

function Datasetupload() {
  
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
    <div className = "dataset-upload-container">
      <h2>Resume Parsing</h2>
      <body>Extract data from candidate resumes</body>
      <input
        type = "file" 
        value={file.resumeParsing}
        onChange={(event) =>
          setFile({ ...file, resumeParsing: event.target.value })
        }
      />
      <br/>
      <h1>OR</h1>
      <h2>Import Dataset</h2>
      <body>Import an existing dataset</body>
      <input
        type = "file"
        value={file.importDataset}
        onChange={(event) =>
          setFile({ ...file, importDataset: event.target.value })
        }
      />
    </div>
  )
}

export default Datasetupload