import React, { useState } from 'react';
import { CheckBox } from "@material-ui/icons";
import "./style.css";
import { Container } from '@mantine/core';

const DynamicTable = () => {
  const data = [
    { id: 1, name: 'SYED MUHAMMAD ALI HAIDER RIZVI', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    { id: 1, name: 'John', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    { id: 1, name: 'John', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    
    { id: 1, name: 'SYED MUHAMMAD ALI HAIDER RIZVI', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    { id: 1, name: 'John', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    { id: 1, name: 'John', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    
    { id: 1, name: 'SYED MUHAMMAD ALI HAIDER RIZVI', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    { id: 1, name: 'John', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    { id: 1, name: 'John', age: 25, city: 'New York' },
    { id: 2, name: 'Jane', age: 30, city: 'Los Angeles' },
    { id: 3, name: 'Alice', age: 22, city: 'Chicago' },
    { id: 4, name: 'Bob', age: 28, city: 'San Francisco' },
    
  ];

  const [selectedRows, setSelectedRows] = useState([]);

  const handleRowSelect = (id) => {
    if (selectedRows.includes(id)) {
      setSelectedRows(selectedRows.filter(rowId => rowId !== id));
    } else {
      setSelectedRows([...selectedRows, id]);
    }
  };

  const handleSelectAll = () => {
    if (selectedRows.length === data.length) {
      setSelectedRows([]);
    } else {
      const allIds = data.map(item => item.id);
      setSelectedRows(allIds);
    }
  };

  return (
    <>
      <div className="row">
        <a href="" target="_blank">
          <img
            src="/Logo_Ezhire.svg"
            className="logo tauri"
            alt="Tauri logo" />
        </a>
      </div>
      
      <h1>Dataset-A</h1>
      {/* <Container className='table-container'> 
      <table className='view-table-head' >
          <thead>
            <tr>
              <th className='sel-btn'>
                <button onClick={handleSelectAll}>Select All</button>
              </th>
              <th className='th-id'>ID</th>
              <th className='th-name'>Name</th>
              <th className='th-age'>Age</th>
              <th className='th-city'>City</th>
            </tr>
          </thead>
          </table>

      </Container>*/ }

      <Container className='table-container'>
      <div className='table-div'>
            

        <table className="view-table" >

        <thead>
            <tr>
              <th className='sel-btn'>
                <button onClick={handleSelectAll}>Select All</button>
              </th>
              <th className='th-id'>ID</th>
              <th className='th-name'>Name</th>
              <th className='th-age'>Age</th>
              <th className='th-city'>City</th>
            </tr>
          </thead>
          

          {data.map((item, index) => (
            <tr key={index}>
              <th className='sel-btn'>
                <input
                  type="checkbox"
                  checked={selectedRows.includes(item.id)}
                  onChange={() => handleRowSelect(item.id)}
                />
              </th>
              <th className='th-id'>{item.id}</th>
              <th className='th-name'>{item.name}</th>
              <th className='th-age'>{item.age}</th>
              <th className='th-city'>{item.city}</th>
            </tr>
          ))}
        </table>
      </div>
      </Container>
    </>
  );
};

export default DynamicTable;
