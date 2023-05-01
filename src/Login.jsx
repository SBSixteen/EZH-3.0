import { useState } from "react";
import React from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import "./style.css";
import { useNavigate } from "react-router-dom";

function Login() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [Remember, setRememberMe] = useState(false);
  const [message, setMessage] = useState(" ");
  const [passwordError, setPasswordError] = useState(" ");
  const [T, setT] = useState(false);
  const [header, setHeader] = useState("Log In");
  const [twofacode, settwofa] = useState("");


  let navigate = useNavigate();
  const gotoRegister = () => {
    let path = "/Register";
    navigate(path);
  };

  const handleLogin = () => {
    const savedUserId = localStorage.getItem("userId"); // retrieve user ID from local storage
    if (savedUserId === name) {
      // authentication successful, proceed with login
    } else {
      // authentication failed, show error message
      setMessage("Incorrect email or password.");
    }
  };

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

  const validatePassword = (password) => {
    return password.length >= 8;
  };

  function validatePass() {
    if (password.length < 8) {
      setPasswordError("Password must be at least 8 characters long.");
    } else {
      setPasswordError("");
    }
  }

  const handlePasswordChange = (event) => {
    setPassword(password.value);
    if (!validatePassword(password.value)) {
    } else {
      setPasswordError(" ");
    }
  };

  return (
    <div className="container">
      <div className="row">
        <a href="" target="_blank">
          <img src="/Logo_Ezhire.svg" className="logo tauri" alt="Tauri logo" />
        </a>
      </div>

      <h1>{header}</h1>


      {!T ? (<React.Fragment>
      <div className="column">
        <form
          onSubmit={(e) => {
            e.preventDefault();
          }}
        >
          <input
            className="default_gap"
            id="username-input"
            onChange={(e) => {
              setName(e.currentTarget.value);
            }}
            placeholder="Enter Username..."
          />

          <br></br>

          <input
            className="default_gap"
            type="password"
            id="password-input"
            onChange={(e) => {
              setPassword(e.currentTarget.value);
            }}
            placeholder="Enter Password..."
          />

          <div className="row">
            <input className="check" type="checkbox"></input>
            <p onChange={(e) => setRememberMe(e.currentTarget.checked)}></p>

            <p>Remember Me?</p>
          </div>

          <button
            className="default_m_right"
            type="submit"
            onClick={() => {
              {
                handleLogin();
                validatePass(password);
              }

              invoke("login_user", { email: name, password: password }).then(
                (message) => {
                  var x = JSON.parse(message);
                  console.log(x);
                  setGreetMsg(x.response);
                  setT(x.two_fa);
                }
              );
            }}
          >
            {" "}
            Sign In{" "}
          </button>

          <button
            type="button"
            onClick={() => {
              gotoRegister();
            }}
          >
            {" "}
            Register{" "}
          </button>

          <div style={{ color: "red" }}>
            {passwordError}

            <br></br>

            {message}
          </div>

          <a href="/pass-reset" target="_self">
            Forgot Password?
          </a>
          <br></br>
          <a href="/register" target="_self">
            Don't Have an Account?
          </a>
        </form>
      </div>
      </React.Fragment>) :(
              <>
              <div style={{
                alignItems: "center"
              }}>
                <input
                  className="default_gap"
                  id="vcode-input"
                  onChange={(e) => settwofa(e.currentTarget.value)}
                  placeholder="Verification Code"
                />
              </div>
              <div
                style={{
                  alignItems: "center"
                }}
              >
                <button
                  onClick={() => {

                    invoke("match_2fa", {
                      email: name,
                      attempt: twofacode,
                    }).then((message) => {
                      var x = JSON.parse(message);
                      setGreetMsg(x.response);
                    });

                  }}
                >
                  Verify
                </button>
              </div>
            </>)
}
      <p>{greetMsg}</p>
    </div>
  );
}

export default Login;
