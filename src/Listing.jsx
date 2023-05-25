import "./listing.css";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react'
import Sidebar from "./Sidebar.jsx";

function Listing() {

  const [data, setData] = useState([]);

  useEffect(() => {

    invoke("fetch_cloud_stats", {
      owner: sessionStorage.getItem("login"),
    }).then((message) => {
      setData(JSON.parse(message));
      console.log(data);
    })

    //Runs only on the first render
  }, []);

  return (
    <div style={{ display: "flex" }}>
      <div className="sidebar-flex">
        <Sidebar />
      </div>
      <div className="Listing" style={{ flex: 85 }}>
        {data.map((item) => {
          return (
            <div className="styleCard">
              <div className="header">
                <div style={{ display: "flex", float: "left" }}><h2>{item.name}</h2></div>

                <div style={{ display: "flex", float: "right" }}><h3>{item.filetype}</h3></div>
              </div>

              <div className="bottom">
                <div style={{ float: "left" }}>
                  <h5>{item.date}</h5> 
                  <div></div>
                  <h4>{item.size}</h4>
                </div>
                <div style={{ float: "right", }}>
                  {
                    item.filetype == "PDF" &&
                    <button className="parsebtn" onClick={()=>
                      {
                        sessionStorage.setItem("fname", `${item.name}.pdf`);
                      }
                    }>Parse</button>
                  }

                  <button className="buttons">Download</button>
                  <button className="deletebtn">Delete</button>

                </div>
              </div>

            </div>
          );
        })}
      </div>
    </div>
  );
}

export default Listing;