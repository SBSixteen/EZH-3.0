import "./listing.css";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import ImageList from "@mui/material/ImageList";
import ImageListItem from "@mui/material/ImageListItem";

function Listing() {
  const [data, setData] = useState([]);

  useEffect(() => {
    sessionStorage.setItem("login", "nabeelmirza79@gmail.com");

    invoke("fetch_cloud_stats", {
      owner: sessionStorage.getItem("login"),
    }).then((message) => {
      setData(JSON.parse(message));

      console.log(data);
    });

    //Runs only on the first render
  }, []);

  return (
    <div className="Listing">
      {/* <table>
        <thead>
          <tr>
            <th>File Name</th>
            <th>File Type</th>
            <th>Size</th>
            <th>Date Created</th>
          </tr>
        </thead>
        <tbody> */}
      <div className="cloudNav">
        <h1>Navbar</h1>
      </div>
      <div className="imagelist">
        <ImageList
          sx={{ width: "100%", height: "80%" }}
          cols={3}
          rowHeight={160}
        >
          {data.map((item) => (
            <ImageListItem key={item.size}>
              <div className="styleCard">
                <div className="header">
                  <div style={{ display: "flex", float: "left" }}>
                    <h2>{item.name}</h2>
                  </div>

                  <div style={{ display: "flex", float: "right" }}>
                    <h3>{item.filetype}</h3>
                  </div>
                  <br></br>
                </div>

                <div className="bottom">
                  <div style={{ float: "left" }}>
                    <h5>{item.date}</h5> <br></br>
                    <h4>{item.size}</h4>
                  </div>
                  <div style={{ float: "right" }}>
                    <button className="buttons">Down</button>
                    <button>Delete</button>
                  </div>
                </div>
              </div>
            </ImageListItem>
          ))}
        </ImageList>
      </div>
      {/* {data.map((item) => {
        return (
          <div className="styleCard">
            <div className="header">
              <div style={{ display: "flex", float: "left" }}>
                <h2>{item.name}</h2>
              </div>

              <div style={{ display: "flex", float: "right" }}>
                <h3>{item.filetype}</h3>
              </div>
              <br></br>
            </div>

            <div className="bottom">
              <div style={{ float: "left" }}>
                <h5>{item.date}</h5> <br></br>
                <h4>{item.size}</h4>
              </div>
              <div style={{ float: "right" }}>
                <button className="buttons">Down</button>
                <button>Delete</button>
              </div>
            </div>
          </div>
        );
      })} */}
      {/* </tbody>
      </table> */}
    </div>
  );
}

export default Listing;
