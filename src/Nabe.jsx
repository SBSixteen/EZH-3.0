import React from "react";
import { Navigate } from "react-router-dom";
import { useNavigate } from "react-router-dom";
import { useState } from "react";

function Nabeel() {
  const [name, setName] = useState("");
  const navigate = useNavigate();
  const handleClick = () => navigate(name);
  return (
    <>
      <input
        className="default_gap"
        id=""
        onChange={(e) => {
          setName(e.currentTarget.value);
        }}
        placeholder="Enter Username..."
      />
      <br></br>
      <button
        type="button"
        onClick={() => {
          window.location.href =  name ;
        }}
      >
        {" "}
        Register{" "}
      </button>
    </>
  );
}
export default Nabeel;
