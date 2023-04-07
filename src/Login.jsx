import { useState } from "react";
import React from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import "./style.css"

function Login() {

const invoke = window.__TAURI__.invoke
  
const [greetMsg, setGreetMsg] = useState("");
const [name, setName] = useState("");
const [password, setPassword] = useState("");
const [Remember, setRememberMe] = useState(false);
const [proceed, setProceed] = useState(false);

let navigate = useNavigate(); 
const routeChange = () =>{ 
  let path = `/Register`; 
  navigate(path);
}

return (
  
      <div className="container">
  
        <div className="row">
          <a href="" target="_blank">
            <img src="/Logo_Ezhire.svg" className="logo tauri" alt="Tauri logo" />
          </a>
        </div>
  
        <h1>Log In</h1>
  
        <div className="column">
          <form
          onSubmit={(e) =>{
  
            e.preventDefault();
          
  
          }}>
  
          <input
              className="default_gap"
              id="username-input"
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="Enter Username..."
          />
          
          <br></br>
          
          <input 
              className="default_gap"
              type = "password"
              id="password-input"
              onChange={(e) => setPassword(e.currentTarget.value)}
              placeholder="Enter Password..."
            />
          
          <div className="row">
          <input className="check" type = "checkbox"></input>
          <p 
          onChange={(e) => setRememberMe(e.currentTarget.checked)}
          ></p>
          
          <p>
            Remember Me?</p>
          </div>
          
          <button className="default_m_right" type="submit" onClick={
            ()=>{

              invoke('authenticate_user', {'e' : name, 'pwd' : password}).then((message)=>{

                console.log(message);

              })

            }
          }> Sign In </button>
          
          <button type = "button" onClick= {routeChange}> Register </button>
          
          <br></br>
         
          <a style={{marginTop: 30 + 'em'}} href="/PassReset" target="_self" >
          Forgot Password?
          </a>
  
          </form>
        </div>
  
        <p>{greetMsg}</p>
      </div>
    );
  }

  export default Login;