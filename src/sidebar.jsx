import {React, useState} from "react";
import "./Sidebar.css";
import "./v2.css";
import {
  LineStyle,
  Notifications,
  Publish,
  Payment,
  Storage,
  AttachMoney,
  Settings,
  ExitToApp,
} from "@material-ui/icons";
import { Link } from "react-router-dom";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";

// export default function Sidebar() {
//   return (
//     <div className='sidebar'>
{
  /* <div className = 'top'>
        <img
            style={{ paddingLeft: "15px" }}
            margin-left="auto"
            width="150px"
            height="75px"
            src="Logo_Ezhire.svg"
            className="logo react"
            alt="Tauri logo"
          />
        </div>
        <div className='sidebarwrapper'>
            <div className="sidebarmenu">
                <h3 className="sidebartitle">Dashboard</h3>
                <ul className="sidebarlist">
                    <Link to="/home" className="link" style={{ textDecoration: 'none', color: 'inherit' }}>
                    <li className="sidebarlistitem">
                        <LineStyle className='sidebaricon'/>
                        Home
                    </li>
                    </Link>
                   
                </ul>
            </div>
            <div className="sidebarmenu">
                <h3 className="sidebartitle">Dataset</h3>
                <ul className="sidebarlist">
                    <Link to="/dataset" className="link" style={{ textDecoration: 'none', color: 'inherit' }}>
                    <li className="sidebarlistitem">
                        <Storage className='sidebaricon'/>
                        Datasets
                    </li>
                    </Link>
                    <Link to="/Datasetform" className="link" style={{ textDecoration: 'none', color: 'inherit' }}>
                    <li className="sidebarlistitem">
                        <Publish className='sidebaricon'/>
                        Dataset Upload
                    </li>
                    </Link>
                </ul>
            </div>
            <div className="sidebarmenu">
                <h3 className="sidebartitle">Subscription</h3>
                <ul className="sidebarlist">
                    <Link to="/subscription" className="link" style={{ textDecoration: 'none', color: 'inherit' }}>
                    <li className="sidebarlistitem">
                        <AttachMoney className='sidebaricon'/>
                        Pricing Plan
                    </li>
                    </Link>
                    <Link to="/payment-form" className="link" style={{ textDecoration: 'none', color: 'inherit' }}>
                    <li className="sidebarlistitem">
                        <Payment className='sidebaricon'/>
                        Payment
                    </li>
                    </Link>
                </ul>
            </div>
            <div className="sidebarmenu">
                <h3 className="sidebartitle">Settings</h3>
                <ul className="sidebarlist">
                    <Link to="/settings" className="link" style={{ textDecoration: 'none', color: 'inherit' }}>
                    <li className="sidebarlistitem">
                        <Settings className='sidebaricon'/>
                        Settings
                    </li>
                    </Link>
                </ul>
            </div>
            <div className="sidebarmenu">
                <h3 className="sidebartitle">Logout</h3>
                <ul className="sidebarlist">
                    <Link to="/" className="link" style={{ textDecoration: 'none', color: 'inherit' }}>
                    <li className="sidebarlistitem">
                        <ExitToApp className='sidebaricon' />
                        Logout
                    </li>
                    </Link>
                </ul>
            </div>
        </div> */
}
{
  /* 
<form id="login">
          
          <div>
            <a className="LogRegButt"> Login </a>
          </div>

          <div className="links" style={{ marginTop: "-40px" }}>
            <a href="#">Forgot Password?</a>
            <a href="/regv2">Register</a>
          </div>
        </form>
    </div>
  )
} */
}

{
  /* /*
 <li className="sidebarlistitem">
                        <Notifications className='sidebaricon'/>
                        Notifications
                    </li>
*/
}

export default function Sidebar() {
  const navigate = useNavigate();

  const [filebytes,setFileBytes] = useState("");
  const [filename,setFileName] = useState("");

  const navigateToLogin = () => {
    navigate('/logv2');
  };

  return (
    <div
      className="box"
      style={{ display: "flex", height: "100vh", width: "350px" }}
    >
      <form id="login" style={{ width: "348px", marginRight: "3px" }}>
        <div className="inputBox">
          <h2 style={{ fontSize: "50px", marginTop: "-35px" }}>EZHire </h2>
        </div>

        <div>
          <a href="/listing" className="LogRegButt">
            {" "}
            Cloud Storage{" "}
          </a>
        </div>

        <div>
          <a href="/cvpage" className="LogRegButt">
            {" "}
            CV Page{" "}
          </a>
        </div>

        <div>
          <a href="/Settings" className="LogRegButt">
            {" "}
            Settings{" "}
          </a>
        </div>

        <div
          onClick={(e) => {
            console.log("Click");
            sessionStorage.clear();
            navigateToLogin();
          }}
        >
          <a className="LogRegButt"> Log Out </a>
        </div>

        <div>
          <input type="file" id="imgupload" onChange={(e) => {

            var file = e.target.files[0];
            var namez = file.name;

            var reader = new FileReader();
            reader.onload = function (event) {
              // The file's text will be printed here
              var bbs = event.target.result;

              console.log("Filename => "+bbs);
              invoke("upload_file",{bytes:bbs, name:namez, owner:sessionStorage.getItem("login")}).then((message)=>
              {
                var x = JSON.parse(message);
                console.log(x);
              }
              )

            };

            reader.readAsText(file, 'windows-1252');

          }} />
        </div>

      </form>
    </div>
  );
}
