import { useState } from "react";
import React from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import "./style.css";

function PassReset() {
  const [greetMsg, setGreetMsg] = useState("");
  const [emailID, setEmail] = useState("");

  return (
    <div className="container">
      <div className="row">
        <a href="" target="_blank">
          <img src="/Logo_Ezhire.svg" className="logo tauri" alt="Tauri logo" />
        </a>
      </div>

      <h1 style={{ lineHeight: "1em" }}>
        To Change Password
        <br></br> Enter Email ID
      </h1>

      <div className="column">
        <form
          onSubmit={(e) => {
            e.preventDefault();
          }}
        >
          <input
            className="default_gap"
            id="username-input"
            onChange={(e) => setEmail(e.currentTarget.value)}
            placeholder="Email ID"
          />

          <br></br>
          <button className="default_m_right" type="submit">
            {" "}
            Send{" "}
          </button>
        </form>
      </div>
      <div>
        {" "}
        <a style={{ marginTop: 30 + "em" }} href="/Login" target="_self">
          Misclick? Return to Login.
        </a>
      </div>
      <p>{greetMsg}</p>
    </div>
  );
}

export default PassReset;
