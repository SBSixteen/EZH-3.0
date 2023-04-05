import React from "react";

function Button(props) {
  
  return (
    <button type="button" onClick={() => {}} style={{ marginTop: "20px" }}>
      {props.title}
    </button>
  );
}

export default Button;
