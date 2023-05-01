import App from "./App.js";
import "./style.css";
import { Container } from "@mantine/core";
import React, { useState } from 'react';
import { Container } from '@mantine/core';
import {
  IconCross,
  IconDatabaseExport,
} from '@tabler/icons-react';
import './style.css';

const Dataset = () => {
  const [showExport, setShowExport] = useState(false);
  const [selectedCard, setSelectedCard] = useState(-1);

  const toggleExport = (index) => {
    setSelectedCard(index);
    setShowExport(true);
  };

  const sets = [
    {
      title: 'MTO',
      noofcvs: '100',
      features: ['Collected By Abdullah'],
    },
    {
      title: 'managerial',
      noofcvs: '50',
      features: ['Collected By Ali'],
    },
    {
      title: 'HR',
      noofcvs: '500',
      features: ['Collected By Nabeel'],
    },
  ];

  const Datacard = ({ title, noofcvs, features, index }) => {
    const handleCrossClick = () => {};

    const handleExportClick = () => {
      toggleExport(index);
    };

    return (
      <div className="Data-card">
        <div className="Data-card-header">
          <h2>{title}</h2>
          <div className="Data-card-icons">
            <button className="iconbutton" onClick={handleCrossClick}>
              <IconCross size={20} />
            </button>
            <button className="iconbutton" onClick={handleExportClick}>
              <IconDatabaseExport size={20} />
            </button>
          </div>
        </div>
        <p className="noofcvs">{noofcvs}</p>
        <ul className="features">
          {features.map((feature, index) => (
            <li key={index}>{feature}</li>
          ))}
        </ul>
      </div>
    );
  };

  return (
    <>
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
            <Datacard key={index} index={index} {...set} />
          ))}
        </div>
      </div>
      <div className="bottom-right-button">
        <button>Add new </button>
      </div>
      {showExport && selectedCard >= 0 && (
        <div className="popup">
          <div className="overlay" onClick={() => setShowExport(false)}></div>
          <div className="popup-content">
            <h2>hello</h2>
            <button className="close-Export" onClick={() => setShowExport(false)}>
              close
            </button>
          </div>
        </div>
      )}
    </>
  );
};

export default Dataset;
