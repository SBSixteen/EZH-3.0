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

  function giveInput() {
    let text = "";

    for (let member in inputRefs) {
      text += member;
    }

    return text;
  }

  return (
    <div className="InputField">
      {Array.from({ length: 7 }, (_, i) => (
        <input
          style={{ width: "50px", marginRight: "10px" }}
          key={i}
          type="text"
          maxLength={1}
          ref={inputRefs[i]}
          onChange={(e) => handleChange(i, e)}
        />
      ))}
    </div>
  );
}

export default InputField;
