import React, { useState } from "react";
import "./v2.css";
import { invoke } from "@tauri-apps/api/tauri";
import {useNavigate} from 'react-router-dom';

function Login_v2() {

  const navigate = useNavigate();

  const to2fa = () => {
    navigate('/2fa');
  };

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [check_e, setcheckE] = useState(false);
  const [check_p, setcheckP] = useState(false);
  const [transit, setTransit] = useState(true);
  const [helptext, sethelptext] = useState("");

  const emailValidation = () => {
    const regEx = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (regEx.test(email)) {
      setcheckE(true);
    } else if (!regEx.test(email)) {
      setcheckE(false);
    } else if (email.length == 0) {
      setcheckE(false);
    }
  };

  const passValidation = () => {
    if (password.length < 7) {
      setcheckP(false);
    } else {
      setcheckP(true);
    }
  };

  const validate = () => {
    emailValidation();
    passValidation();

    if (!check_p) {
      sethelptext("Password is too short.");
    }

    if (!check_e) {
      sethelptext("Email is invalid");
      if (!check_p) {
        sethelptext("Email & Password are invalid.");
      }
    }

    if (check_p && check_e) {
      sethelptext("");
    }
    console.log(helptext);
  };

  return (
    <div>
      <div>
        {/* {Someone get the logo loaded!!} */}
        <img src="/EZH Logo.svg" style={{ width: "375px" }} />
      </div>

      <title>EZHIRE</title>
     <center> <div className="box">
        <form id="login">
          <div className="inputBox">
            <h2>Sign In</h2>
            <input
              type="text"
              required
              onChange={(e) => {
                setEmail(e.currentTarget.value);
                validate();
              }}
            ></input>
            <span>Email</span>
            <i />
          </div>

          <div className="inputBox">
            <input
              type="password"
              style={{ marginTop: "-25px" }}
              required
              onChange={(e) => {
                setPassword(e.currentTarget.value);
                validate();
              }}
            ></input>
            <span>Enter Password</span>

            <i />
          </div>

          <div
            onClick={(e) => {
              if (check_e && check_p && transit) {
                setTransit(false);
                sethelptext("Querying...");

                invoke("login_user", { email: email, password: password }).then(
                  (message) => {
                    var x = JSON.parse(message);
                    console.log(x);
                    sethelptext(x.response);

                    if (x.value) {
                      sessionStorage.setItem("login", email);
                      if(x.two_fa){
                        to2fa();
                      }else{

                      }
                    }

                    setTransit(true);
                  }
                );
                console.log("end of task");
              }
            }}
          >
            <a className="LogRegButt"> Login </a>
          </div>

          <div className="links" style={{ marginTop: "-40px" }}>
            <a href="#">Forgot Password?</a>
            <a href="/regv2">Register</a>
          </div>

          <a style={{ color: "#FFFFFF", marginTop: "-10px" }}> {helptext} </a>
        </form>
      </div></center>
    </div>
  );
}

export default Login_v2;
