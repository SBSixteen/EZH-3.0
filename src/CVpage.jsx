import React, { useState } from "react";
import Sidebar from "./Sidebar";
import pieChart from "./Components/PieChart";
import PieChart from "./Components/PieChart/PieChart";
import { ResponsivePie } from "@nivo/pie";

function Cvpage() {
  //   const [active, setActive] = useState("");

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
            <form id="login"></form>
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
          <div className="GitChart" style={{ height: "100%", width: "50%" }}>
            <div className="card" style={{ height: "100%" }}>
              <form id="login">
                <ResponsivePie
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
                />
              </form>
            </div>
          </div>
          <div className="CVSkills" style={{ height: "100%", width: "50%" }}>
            <div className="card" style={{ height: "100%" }}>
              <form id="login"></form>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
export default Cvpage;
