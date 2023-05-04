import App from "./App.js";
import "./style.css";
import "./dataset.css";
import Sidebar from "./Sidebar.jsx";
import "./Home.css";

import { Container } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import React, { useState } from 'react';
import {
  IconTrash,
  IconDatabaseExport,
  IconArrowBadgeUp,
  IconX,
} from "@tabler/icons-react";
import "./style.css";

const Dataset = () => {
  const [showRemove, setShowRemove] = useState(false);
  const [showExport, setShowExport] = useState(false);
  const [selectedCard, setSelectedCard] = useState(-1);


  let navigate = useNavigate();
const gotoaddnew = () =>{

  let path = "/Datasetform";
  navigate(path);
  
}
let navigate2 = useNavigate();
const gototable = () =>{
  let path2 = "/DynamicTable";
  navigate(path2);

}

  const toggleExport = (index, check) => {
    setSelectedCard(index);
    setShowExport(true);
  };

  const toggleRemove = (index) => {
    setSelectedCard(index);
    setShowRemove(true);
  };

  const sets = [
    {
      title: "Martin Dow Hiring",
      noofcvs: "100",
      features: ["For MTO position", "GooGoo"],
      check: "app",
    },
    {
      title: "Stratesfy WebDev Intern",
      noofcvs: "50",
      features: ["For Accounts officer", "CFA"],
      check: "app",
    },
    {
      title: "IBA TA",
      noofcvs: "77",
      features: ["Logistics"],
      check: "appleasdasdasdasdasdasdasddasdas",
    },
    {
      title: "CEO of Valider",
      noofcvs: "3",
      features: ["Head of procurement"],
      check: "appleasdasdasdasdasdasdasddasdas",
    },
  ];

  const Datacard = ({ title, noofcvs, check, features, index }) => {
    const handleCrossClick = () => {
      toggleRemove(index);
    };

    const handleExportClick = () => {
      toggleExport(index, check);
    };

    return (
      <div className="Data-card">
        <div style={{ height: "50px" }}>
          <div style={{ width: "45%", float: "left" ,  display:"flex",marginLeft:"2.5%", marginTop:"10px"}}><h2>{title}</h2></div>
          <div style={{ width: "50%", float: "right" , display:"flex", marginRight:"2.5%", justifyContent:"flex-end"}}>
            <button className="iconbutton" onClick={handleCrossClick}>
              <IconTrash size={20} />
            </button>
            <button className="iconbutton" onClick={handleExportClick}>
              <IconDatabaseExport size={20} />
            </button>
            <button className="iconbutton" onClick={gototable}>
              <IconArrowBadgeUp size={20} />
            </button>
          </div>
        </div>
        <p className="noofcvs" style={{ display:"flex",marginLeft:"2.5%", marginTop:"10px",}}>{noofcvs}</p>
        <p className="features" style={{marginLeft:"2.5%"}}>
          {features.map((feature, index) => (
            <li key={index}>{feature}</li>
          ))}
        </p>
      </div>
    );
  };

  return (
    <>
    <div className="container-new">
      <Sidebar/>
      <div className="others">
      <Container>
        <div className="row">
          <a href="" target="_blank">
            <img
              src="/Logo_Ezhire.svg"
              className="logo tauri"
              alt="Tauri logo"
            />
          </a>
        </div>
      </Container>
      <div className="window">
        <div className="Data-set">
          {sets.map((set, index) => (
            <Datacard key={index} index={index} {...set} check={set.check} />
          ))}
        </div>
      </div>
      {showExport && selectedCard >= 0 && (
        <div className="popup">
          <div className="overlay" onClick={() => setShowExport(false)}></div>
          <div className="popup-content">
            <h className="exp-title">Export as</h>
            <p>{sets[selectedCard].check}</p>

            <div className="export-as">
              <input className="check" type="checkbox"></input>
              <p>.CSV</p>
            </div>
            <div className="export-as">
              <input className="check" type="checkbox"></input>
              <p>.MDB</p>
            </div>

            <div className="export-as">
              <input className="check" type="checkbox"></input>
              <p>.XLSX</p>
            </div>

            <div className="export-as">
              <input className="check" type="checkbox"></input>
              <p>.XML</p>
            </div>

            <div className="export-as">
              <input className="check" type="checkbox"></input>
              <p>.SQL</p>
            </div>

            <div className="export-as">
              <input className="check" type="checkbox"></input>
              <p>.MONGO</p>
            </div>

            <div className="export-as">
              <input className="check" type="checkbox"></input>
              <p>select shortlisted candidates only</p>
            </div>

            <div>
              <input
                className="eport-name"
                type="text"
                placeholder="File Name"
              ></input>
            </div>

            <button className="default_m_right" type="submit">
              {" "}
              Export{" "}
            </button>

            <button
              className="close-Export"
              onClick={() => setShowExport(false)}
            >
              <IconX size={20} />
            </button>
          </div>
        </div>
      )}
      {showRemove && selectedCard >= 0 && (
        <div className="popup">
          <div className="overlay" onClick={() => setShowExport(false)}></div>
          <div className="popup-content">
            <div>
            <h className="quit-msg">Are you sure you want to delete</h>
            </div>       
                
                <button className="remove-btn" type="submit"
                onClick={() =>setShowRemove(false)}> YES </button>
                
                <button className="remove-btn" type="submit"
                onClick={() =>setShowRemove(false)}>   NO </button>


            
            <button className="close-Export" onClick={() => setShowRemove(false)}>
              <IconX size={20}/>
            </button>
          </div>
        </div>
      )}
      </div>
    </div>
    </>
  );
};

export default Dataset;
