import React from 'react';

function DatasetInfo({ formData, setFormData }) {
  return (
    <div className = "dataset-info-container" >
      <input 
        type = "text" 
        placeholder = "Dataset Name..."
        value={formData.datasetName}
        onChange={(event) =>
          setFormData({ ...formData, datasetName: event.target.value })
        }
      />
      <input 
        type = "datetime-local"
        value={formData.deadline}
        onChange={(event) =>
          setFormData({ ...formData, deadline: event.target.value })
        }
      />
    </div>
  )
}

export default DatasetInfo