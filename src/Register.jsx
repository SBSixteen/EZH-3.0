import { useState } from "react";
import React from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import "./style.css";
// import { Button } from "./Components/Button";
import Button from "./Components/Button/";

import { resolvePath, useNavigate } from "react-router-dom";

function Register() {
  const [greetMsg, setGreetMsg] = useState("");
  const [emailID, setEmail] = useState("");
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [response, setResponse] = useState("");
  const [Remember, setRememberMe] = useState(false);
  const [T, setT] = useState(false);
  const [title, setTitle] = useState("Registration");
  return (
    <div className="container">
      <div className="row">
        <a href="" target="_blank">
          <img src="/Logo_Ezhire.svg" className="logo tauri" alt="Tauri logo" />
        </a>
      </div>

      <h1>{title}</h1>

      <div
        className="column"
        style={{
          display: "flex",
          flex: 1,
          alignItems: "center",
        }}
      >
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

          {!T ? (
            <React.Fragment>
              <input
                className="default_gap"
                id="username-input"
                onChange={(e) => setName(e.currentTarget.value)}
                placeholder="Entrer Username"
              />
              <br></br>
              <input
                className="default_gap"
                type="Password"
                id="password-input"
                onChange={(e) => setPassword(e.currentTarget.value)}
                placeholder="Password"
              />
              <br></br>
              <input
                className="default_gap"
                type="Confirm Password"
                id="password-input"
                onChange={(e) => setPassword(e.currentTarget.value)}
                placeholder="Confirm Password"
              />

              <br></br>

              <button
                className="default_m_right"
                type="submit"
                onClick={() => {
                  invoke("create_user", { mail: emailID, pwd: password }).then(
                    (message) => {
                      setResponse(message);
                      console.log(message);
                      var x = JSON.parse(message);
                      setGreetMsg(x.response);
                      console.log(x.value);
                      setT(x.value);
                      console.log("Toggle = ", T);
                      setTitle("Verify Email");
                    }
                  );
                }}
              >
                {" "}
                Register
              </button>
            </React.Fragment>
          ) : (
            <>
              <div style={{ display: "flex", flexDirection: "row" }}>
                <button
                  onClick={() => {
                    setT((T) => !T);
                    setTitle("Registration");
                    setGreetMsg("");
                  }}
                >
                  back
                </button>

                <button
                  style={{ margin: "100" }}
                  onClick={() => {
                    setT((T) => !T);
                    setTitle("Verify Email");
                    setGreetMsg("");
                  }}
                >
                  Verify
                </button>
              </div>
            </>
          )}

          <br></br>
        </form>
        <div>
          <a
            style={{ marginTop: 30 + "em" }}
            href="/Login"
            target="_self"
            onClick={() => {}}
          >
            Already have an account?
          </a>
        </div>
      </div>

      <p>{greetMsg}</p>
    </div>
  );
}

export default Register;
