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
  const [cpassowrd, setCPassword] = useState("");
  const [response, setResponse] = useState("");
  const [Remember, setRememberMe] = useState(false);
  const [T, setT] = useState(false);
  const [title, setTitle] = useState("Registration");
  const [message, setMessage] = useState("");
  const [passwordError, setPasswordError] = useState("");
  const [matchError, setMatchError] = useState("");

  
  const emailValidation = () => {
    const regEx = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (regEx.test(name)) {
      setMessage(" ");
    } else if (!regEx.test(name) && name !== " ") {
      setMessage("email or password incorrect");
    } else {
      setMessage(" ");
    }
  };

  function validatePass() {
    if (password.length < 8) {
      setPasswordError("Password must be at least 8 characters long.");
    } else {
      setPasswordError("");
    }
  }

  function ComparePass() {
    if(cpassowrd != password){
    setMatchError("Passwords do not match!");
    }else{
     setMatchError(""); 
    }
  }

  let navigate = useNavigate();
  const gotoLogin = () => {
    let path = "/Login";
    navigate(path);
  };


  return (
    <div className="container">
      <div className="row">
        <a href="" target="_blank">
          <img src="/Logo_Ezhire.svg" className="logo tauri" alt="Tauri logo" />
        </a>
      </div>

      <h1>{title}</h1>
      <div>
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
            <br></br>

            {!T ? (
              <React.Fragment>
                <input
                  className="default_gap"
                  id="username-input"
                  onChange={(e) => setEmail(e.currentTarget.value)}
                  placeholder="Email ID"
                />
                <br></br>
                <input
                  className="default_gap"
                  id="username-input"
                  onChange={(e) => setName(e.currentTarget.value)}
                  placeholder="Enter Name"
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
                  onChange={(e) => setCPassword(e.currentTarget.value)}
                  placeholder="Confirm Password"
                />

              <div style={{color:'red'}}>
                <div>{message}</div>
                
                <div>{passwordError}</div>
                
                <div>{matchError} </div>
                </div>
                <button
                  margin="100px"
                  className="default_m_right"
                  type="submit"
                  onClick={() => {

                    ComparePass();
                    validatePass();
                    emailValidation();
                    invoke("create_user", {
                      mail: emailID,
                      username: name,
                      pwd: password,
                    }).then((message) => {
                      setResponse(message);
                      console.log(message);
                      var x = JSON.parse(message);
                      setGreetMsg(x.response);
                      console.log(x.value);
                      setT(x.value);
                      console.log("Toggle = ", T);
                      setTitle("Verify Email");
                    });
                  }}
                >
                  {" "}
                  Register
                </button>
              </React.Fragment>
            ) : (
              <>
                <div style={{ display: "flex", flexDirection: "column" }}>
                  <input
                    className="default_gap"
                    id="username-input"
                    onChange={(e) => setEmail(e.currentTarget.value)}
                    placeholder="7 Digit Code"
                  />
                  <button
                    className="default_m_right"
                    style={{ margin: "100" }}
                    onClick={() => {
                      setT((T) => !T);
                      setTitle("Verify Email");
                      setGreetMsg("");
                    }}
                  >
                    Verify
                  </button>

                  <button
                    className="default_m_right"
                    onClick={() => {
                      setT((T) => !T);
                      setTitle("Registration");
                      setGreetMsg("");
                    }}
                  >
                    Back
                  </button>
                </div>
              </>
            )}

            <br></br>
          </form>
          <div>
            <a
              style={{ marginTop: "30em" }}
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
    </div>
  );
}

export default Register;