import React, { useRef } from "react";

function InputField() {
  const inputRefs = [
    useRef(null),
    useRef(null),
    useRef(null),
    useRef(null),
    useRef(null),
    useRef(null),
    useRef(null),
  ];

  const handleChange = (index, e) => {
    const { value, maxLength } = e.target;
    const nextIndex = index + 1;

    if (value.length >= maxLength && inputRefs[nextIndex]) {
      inputRefs[nextIndex].current.focus();
    }
  };

  return (
    <div className="InputField">
      {Array.from({ length: 7 }, (_, i) => (
        <input
          style={{ width: "50px", marginRight: "10px"}}
          key={i}
          type="text"
          inputMode="numeric"
          pattern="[0-9]*"
          maxLength={1}
          ref={inputRefs[i]}
          onChange={(e) => handleChange(i, e)}
        />
      ))}
    </div>
  );
}

export default InputField;
