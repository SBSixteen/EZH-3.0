import React, { useState } from "react";
import Sidebar from "./Sidebar";
import pieChart from "./Components/PieChart";
import PieChart from "./Components/PieChart/PieChart";
import { ResponsivePie } from "@nivo/pie";

function Cvpage() {
  //   const [active, setActive] = useState("");
  const cv = {
    name: "Nabeel Mirza",
    phone: "+92 323 8227006",
    email: "nabeelmirza79@gmail.com",
    skills: {
      Reqwest: 2,
      Express: 1,
      "C++": 1,
      Java: 2,
      SQL: 1,
      Rust: 4,
      React: 2,
      MongoDB: 1,
    },
    institutes: ["Institute of Business Administration", "Nixor College"],
    workexp: { name: [], years: [] },
    git_repos: [
      {
        name: "100-days-of-javascript",
        description:
          "AsmrProg Youtube Channel 100 days of javascript codes collection",
        url: "https://github.com/SBSixteen/100-days-of-javascript",
        score: { CSS: 55266, JavaScript: 70150, HTML: 35524 },
      },
      {
        name: "Academia",
        description: "For all your exam needs",
        url: "https://github.com/SBSixteen/Academia",
        score: { Java: 34707 },
      },
      {
        name: "dataBased",
        description:
          "A Based database built in Rust. It is a complete DB solution",
        url: "https://github.com/SBSixteen/dataBased",
        score: { Rust: 51195 },
      },
      {
        name: "Desolation-Sins-of-Mankind",
        description: "A 2D top-down shooter made on C++ using SDL2.",
        url: "https://github.com/SBSixteen/Desolation-Sins-of-Mankind",
        score: { C: 503, "C++": 44129 },
      },
      {
        name: "DriveTrain",
        description:
          "Your recent addiction can take you a long way. It's very useful for people with big garages and lots of cars!",
        url: "https://github.com/SBSixteen/DriveTrain",
        score: { Java: 11154 },
      },
      {
        name: "EZH-2.0",
        description: "hehaw",
        url: "https://github.com/SBSixteen/EZH-2.0",
        score: { HTML: 2365, Rust: 2617, CSS: 3161, JavaScript: 17589 },
      },
      {
        name: "EZH-3.0",
        description: "EZHire Reborn",
        url: "https://github.com/SBSixteen/EZH-3.0",
        score: { CSS: 30311, Rust: 166456, HTML: 1721, JavaScript: 142585 },
      },
      {
        name: "EZHire",
        description: "HR tools",
        url: "https://github.com/SBSixteen/EZHire",
        score: {},
      },
      {
        name: "Firewhirl-x-Mindstorm",
        description: "DNE (Does Not Exist)",
        url: "https://github.com/SBSixteen/Firewhirl-x-Mindstorm",
        score: {},
      },
      {
        name: "ISAE_Project",
        description: "DNE (Does Not Exist)",
        url: "https://github.com/SBSixteen/ISAE_Project",
        score: { "C++": 25197 },
      },
      {
        name: "Jatabase",
        description: "An alternative to using SQL, for Java.",
        url: "https://github.com/SBSixteen/Jatabase",
        score: { Java: 14885 },
      },
      {
        name: "Plane-Game",
        description: "A simple plane game made using SDL2.",
        url: "https://github.com/SBSixteen/Plane-Game",
        score: { C: 175, "C++": 31880 },
      },
      {
        name: "PowerPlay",
        description:
          "An Offline Cricket Self Tailored Database Solution. Optimal for Schools/Universities ",
        url: "https://github.com/SBSixteen/PowerPlay",
        score: { Java: 552239 },
      },
      {
        name: "RookieGame-FireWhirl",
        description: "A Repo for a HCG made in Unity during the Gam Jam 2022.",
        url: "https://github.com/SBSixteen/RookieGame-FireWhirl",
        score: {
          "C#": 9205844,
          HLSL: 21151,
          "Objective-C++": 8982,
          ShaderLab: 82906,
          "C++": 17509,
          CMake: 426,
        },
      },
    ],
    github: "Github.com/SBSixteen",
    git_username: "sbsixteen",
    linkedin: "",
    template: "Software Developer (Junior)",
    tokens: [],
  };

  const data = [
    {
      id: "c",
      label: "c",
      value: 414,
      color: "hsl(115, 70%, 50%)",
    },
    {
      id: "elixir",
      label: "elixir",
      value: 83,
      color: "hsl(120, 70%, 50%)",
    },
    {
      id: "javascript",
      label: "javascript",
      value: 189,
      color: "hsl(94, 70%, 50%)",
    },
    {
      id: "haskell",
      label: "haskell",
      value: 198,
      color: "hsl(212, 70%, 50%)",
    },
    {
      id: "ruby",
      label: "ruby",
      value: 15,
      color: "hsl(144, 70%, 50%)",
    },
  ];

  return (
    <div style={{ display: "flex" }}>
      <div className="sidebar-flex">
        <Sidebar />
      </div>

      <div style={{ display: "flex", width: "85%", flexDirection: "column" }}>
        <div className="topCVstats" style={{ height: "50%", width: "100%" }}>
          <div className="card" style={{ height: "100%" }}>
            <form id="login">
              <h2 style = {{textAlign:"center"}}>CV Information</h2>
              <p>Name: {cv.name}</p>
              <p>Phone: {cv.phone}</p>
              <p>Email: {cv.email}</p>

              <h3 style = {{textAlign:"center"}}>Institutes</h3>
              <ul>
                {cv.institutes.map((institute, index) => (
                 <p> <li  key={index}>{institute}</li> </p>
                ))}
              </ul>
              
            </form>
          </div>
        </div>

        <div
          className="bottomCVstats"
          style={{
            height: "50%",
            width: "100%",
            display: "flex",
            flexDirection: "row",
          }}
        >
          <div className="GitChart" style={{ height: "100%", width: "50%",overflow: 'auto',}}>
            <div className="card" style={{ height: "100%" }}>
              <form id="login" style={{overflowY: "scroll"}}>
      <h3 style = {{textAlign:"center"}}>GitHub Repositories</h3>
      <ul>
        {cv.git_repos.map((repo, index) => (
          <li style = {{color:"white"}} key={index}>
            <p>Name: {repo.name}</p>
            <p>Description: {repo.description}</p>
            <p>URL: {repo.url}</p>
            <p>
              Score:{' '}
              {Object.entries(repo.score).map(([language, score]) => (
                <span key={language}>
                  {language}: {score}{' '}
                </span>
              ))}
            </p>
          </li>
        ))}
      </ul>
                {/* <ResponsivePie
                  data={data}
                  margin={{ top: 40, right: 80, bottom: 80, left: 80 }}
                  innerRadius={0.5}
                  padAngle={0.7}
                  cornerRadius={3}
                  activeOuterRadiusOffset={8}
                  borderWidth={1}
                  borderColor={{
                    from: "color",
                    modifiers: [["darker", 0.2]],
                  }}
                  arcLinkLabelsSkipAngle={10}
                  arcLinkLabelsTextColor="#333333"
                  arcLinkLabelsThickness={2}
                  arcLinkLabelsColor={{ from: "color" }}
                  arcLabelsSkipAngle={10}
                  arcLabelsTextColor={{
                    from: "color",
                    modifiers: [["darker", 2]],
                  }}
                  defs={[
                    {
                      id: "dots",
                      type: "patternDots",
                      background: "inherit",
                      color: "rgba(255, 255, 255, 0.3)",
                      size: 4,
                      padding: 1,
                      stagger: true,
                    },
                    {
                      id: "lines",
                      type: "patternLines",
                      background: "inherit",
                      color: "rgba(255, 255, 255, 0.3)",
                      rotation: -45,
                      lineWidth: 6,
                      spacing: 10,
                    },
                  ]}
                  fill={[
                    {
                      match: {
                        id: "ruby",
                      },
                      id: "dots",
                    },
                    {
                      match: {
                        id: "c",
                      },
                      id: "dots",
                    },
                    {
                      match: {
                        id: "go",
                      },
                      id: "dots",
                    },
                    {
                      match: {
                        id: "python",
                      },
                      id: "dots",
                    },
                    {
                      match: {
                        id: "scala",
                      },
                      id: "lines",
                    },
                    {
                      match: {
                        id: "lisp",
                      },
                      id: "lines",
                    },
                    {
                      match: {
                        id: "elixir",
                      },
                      id: "lines",
                    },
                    {
                      match: {
                        id: "javascript",
                      },
                      id: "lines",
                    },
                  ]}
                  legends={[
                    {
                      anchor: "bottom",
                      direction: "row",
                      justify: false,
                      translateX: 0,
                      translateY: 56,
                      itemsSpacing: 0,
                      itemWidth: 100,
                      itemHeight: 18,
                      itemTextColor: "#999",
                      itemDirection: "left-to-right",
                      itemOpacity: 1,
                      symbolSize: 18,
                      symbolShape: "circle",
                      effects: [
                        {
                          on: "hover",
                          style: {
                            itemTextColor: "#000",
                          },
                        },
                      ],
                    },
                  ]}
                /> */}
              </form>
            </div>
          </div>
          <div className="CVSkills" style={{ height: "100%", width: "50%" }}>
            <div className="card" style={{ height: "100%" }}>
              <form id="login">
              <h3 style = {{textAlign:"center"}} >Skills</h3>
              <ul>
                {Object.entries(cv.skills).map(([skill, level]) => (
                  <li style = {{color:"white"}} key={skill}>
                    <p>{skill}: {level} </p>
                  </li>
                ))}
              </ul>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
export default Cvpage;
