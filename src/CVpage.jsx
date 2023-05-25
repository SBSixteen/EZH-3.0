import React, { useState } from "react";
import Sidebar from "./Sidebar";


function Cvpage() {
  //   const [active, setActive] = useState("");


  return (
    <div style={{ display: "flex" }}>

      <div className="sidebar-flex">
        <Sidebar />
      </div>

      <div style={{ display: "flex", width: "85%", flexDirection: "column" }}>

        <div className="topCVstats" style={{ height: "50%", width: "100%" }}>
          <div className="card" style={{height:"100%"}} >
            <form id="login"></form>
          </div>
        </div>

        <div className="bottomCVstats" style={{ height: "50%", width: "100%", display: "flex", flexDirection: "row" }}>
          <div className="GitChart" style={{ height: "100%", width: "50%" }}>
            <div className="card" style={{height:"100%"}}>
              <form id="login"></form>
            </div>
          </div>
          <div className="CVSkills" style={{ height: "100%", width: "50%" }}>
            <div className="card" style={{height:"100%"}} >
              <form id="login" ></form>
            </div>
          </div>
        </div>

      </div>
    </div>

  )
}
export default Cvpage;