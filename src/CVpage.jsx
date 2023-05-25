import React, { useState } from "react";
import Sidebar from "./Sidebar";


function Cvpage() {
//   const [active, setActive] = useState("");


  return (
    <div style={{ display: "flex" }}>
      <div className="sidebar-flex">
        <Sidebar />
      </div>
   
      <div style={{ display: "flex" ,width :"85%", flexDirection : "column"}}>
      <div className="card" >
        <form id="login"></form>
        </div>
      </div>

      <div style={{ display: "flex" ,width :"85%" }}>
      <div className="card" >
        <form id="login"></form>
        </div>
      </div>
      </div>

  )
}
export default Cvpage;