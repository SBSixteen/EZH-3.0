import React, { useState } from 'react';

const NumberInputs = () => {
  const [inputValues, setInputValues] = useState(['', '', '', '', '', '', '']);

  const handleInputChange = (index, value) => {
    if (!isNaN(value)) {
      setInputValues(prevValues => {
        const newValues = [...prevValues];
        newValues[index] = value;
        return newValues;
      });
    }
  };

  return (
    <div style={{ display: 'flex' }}>
      {inputValues.map((value, index) => (
        <input maxLength={1}
          key={index}
          type="text"
          value={value}
          onChange={e => handleInputChange(index, e.target.value)}
          style={{ width: '50px', marginRight: '10px' }}
        />
      ))}
    </div>
  );
};

export default NumberInputs;