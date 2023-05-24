// import React, { useState, useRef, useEffect } from 'react';
// import Chart from 'chart.js/auto';
// import './style.css';

// function Comparison() {
//   const [activeSheet, setActiveSheet] = useState(0);
//   const [users, setUsers] = useState([
//     {
//         name: 'Alice',
//         age: 30,
//         email: 'alice@example.com',
//         income: [
//           { name: 'Salary', value: 5000 },
//           { name: 'B2B', value: 2000 },
//           { name: 'Investments', value: 10000 },
//           { name: 'abbakapaisa', value: 500 },
//         ],
//       },
//       {
//         name: 'Bob',
//         age: 35,
//         email: 'bob@example.com',
//         income: [
//           { name: 'Salary', value: 7000 },
//           { name: 'Bonus', value: 1500 },
//         ],
//       },
//       {
//         name: 'Charlie',
//         age: 25,
//         email: 'charlie@example.com',
//         income: [
//           { name: 'abbakapaisa', value: 6000 },
//           { name: 'stocks', value: 3000 },
//           { name: 'jugaar', value: 8000 },
//         ],
//       },
//       {
//         name: 'David',
//         age: 32,
//         email: 'david@example.com',
//         income: [
//           { name: 'Salary', value: 6000 },
//           { name: 'Bonus', value: 2500 },
//           { name: 'Stocks', value: 9000 },
//         ],
//       },
//       {
//         name: 'Emily',
//         age: 28,
//         email: 'emily@example.com',
//         income: [
//           { name: 'Salary', value: 4000 },
//           { name: 'Consulting', value: 3000 },
//           { name: 'Freelance', value: 7000 },
//         ],
//       },
//       {
//         name: 'Frank',
//         age: 40,
//         email: 'frank@example.com',
//         income: [
//           { name: 'Salary', value: 8000 },
//           { name: 'Stocks', value: 2000 },
//           { name: 'Real Estate', value: 12000 },
//         ],
//       },
//       {
//         name: 'Grace',
//         age: 27,
//         email: 'grace@example.com',
//         income: [
//           { name: 'Salary', value: 4500 },
//           { name: 'Consulting', value: 3500 },
//           { name: 'Investments', value: 9000 },
//         ],
//       },
//       {
//         name: 'Harry',
//         age: 31,
//         email: 'harry@example.com',
//         income: [
//           { name: 'Salary', value: 6500 },
//           { name: 'B2B', value: 1500 },
//           { name: 'Stocks', value: 8000 },
//         ],
//       },
//       {
//         name: 'Isabella',
//         age: 29,
//         email: 'isabella@example.com',
//         income: [
//           { name: 'Salary', value: 5000 },
//           { name: 'Bonus', value: 1000 },
//           { name: 'Freelance', value: 6000 },
//         ],
//       },
//       {
//         name: 'John',
//         age: 33,
//         email: 'john@example.com',
//         income: [
//           { name: 'Salary', value: 7000 },
//           { name: 'Stocks', value: 5000 },
//           { name: 'Investments', value: 8000 },
//         ],
//       },{
//         name: 'Alice',
//         age: 30,
//         email: 'alice@example.com',
//         income: [
//           { name: 'Salary', value: 5000 },
//           { name: 'B2B', value: 2000 },
//           { name: 'Investments', value: 10000 },
//         ],
//       },
//       {
//         name: 'Bob',
//         age: 35,
//         email: 'bob@example.com',
//         income: [
//           { name: 'Salary', value: 7000 },
//           { name: 'Bonus', value: 1500 },
//           { name: 'Dividend', value: 5000 },
//         ],
//       },
//       {
//         name: 'Charlie',
//         age: 25,
//         email: 'charlie@example.com',
//         income: [
//           { name: 'abbakapaisa', value: 6000 },
//           { name: 'stocks', value: 3000 },
//           { name: 'jugaar', value: 8000 },
//         ],
//       },
//       {
//         name: 'David',
//         age: 32,
//         email: 'david@example.com',
//         income: [
//           { name: 'Salary', value: 6000 },
//           { name: 'Bonus', value: 2500 },
//           { name: 'Stocks', value: 9000 },
//         ],
//       },
//       {
//         name: 'Emily',
//         age: 28,
//         email: 'emily@example.com',
//         income: [
//           { name: 'Salary', value: 4000 },
//           { name: 'Consulting', value: 3000 },
//           { name: 'Freelance', value: 7000 },
//         ],
//       },
//       {
//         name: 'Frank',
//         age: 40,
//         email: 'frank@example.com',
//         income: [
//           { name: 'Salary', value: 8000 },
//           { name: 'Stocks', value: 2000 },
//           { name: 'Real Estate', value: 12000 },
//         ],
//       },
//       {
//         name: 'Grace',
//         age: 27,
//         email: 'grace@example.com',
//         income: [
//           { name: 'Salary', value: 4500 },
//           { name: 'Consulting', value: 3500 },
//           { name: 'Investments', value: 9000 },
//         ],
//       },
//       {
//         name: 'Harry',
//         age: 31,
//         email: 'harry@example.com',
//         income: [
//           { name: 'Salary', value: 6500 },
//           { name: 'B2B', value: 1500 },
//           { name: 'Stocks', value: 8000 },
//         ],
//       },
//       {
//         name: 'Isabella',
//         age: 29,
//         email: 'isabella@example.com',
//         income: [
//           { name: 'Salary', value: 5000 },
//           { name: 'Bonus', value: 1000 },
//           { name: 'Freelance', value: 6000 },
//         ],
//       },
//       {
//         name: 'John',
//         age: 33,
//         email: 'john@example.com',
//         income: [
//           { name: 'Salary', value: 7000 },
//           { name: 'Stocks', value: 5000 },
//           { name: 'Investments', value: 8000 },
//         ],
//       },{
//         name: 'Alice',
//         age: 30,
//         email: 'alice@example.com',
//         income: [
//           { name: 'Salary', value: 5000 },
//           { name: 'B2B', value: 2000 },
//           { name: 'Investments', value: 10000 },
//         ],
//       },
//       {
//         name: 'Bob',
//         age: 35,
//         email: 'bob@example.com',
//         income: [
//           { name: 'Salary', value: 7000 },
//           { name: 'Bonus', value: 1500 },
//           { name: 'Dividend', value: 5000 },
//         ],
//       },
//       {
//         name: 'Charlie',
//         age: 25,
//         email: 'charlie@example.com',
//         income: [
//           { name: 'abbakapaisa', value: 6000 },
//           { name: 'stocks', value: 3000 },
//           { name: 'jugaar', value: 8000 },
//         ],
//       },
//       {
//         name: 'David',
//         age: 32,
//         email: 'david@example.com',
//         income: [
//           { name: 'Salary', value: 6000 },
//           { name: 'Bonus', value: 2500 },
//           { name: 'Stocks', value: 9000 },
//         ],
//       },
//       {
//         name: 'Emily',
//         age: 28,
//         email: 'emily@example.com',
//         income: [
//           { name: 'Salary', value: 4000 },
//           { name: 'Consulting', value: 3000 },
//           { name: 'Freelance', value: 7000 },
//         ],
//       },
//       {
//         name: 'Frank',
//         age: 40,
//         email: 'frank@example.com',
//         income: [
//           { name: 'Salary', value: 8000 },
//           { name: 'Stocks', value: 2000 },
//           { name: 'Real Estate', value: 12000 },
//         ],
//       },
//       {
//         name: 'Grace',
//         age: 27,
//         email: 'grace@example.com',
//         income: [
//           { name: 'Salary', value: 4500 },
//           { name: 'Consulting', value: 3500 },
//           { name: 'Investments', value: 9000 },
//         ],
//       },
//       {
//         name: 'Harry',
//         age: 31,
//         email: 'harry@example.com',
//         income: [
//           { name: 'Salary', value: 6500 },
//           { name: 'B2B', value: 1500 },
//           { name: 'Stocks', value: 8000 },
//         ],
//       },
//       {
//         name: 'Isabella',
//         age: 29,
//         email: 'isabella@example.com',
//         income: [
//           { name: 'Salary', value: 5000 },
//           { name: 'Bonus', value: 1000 },
//           { name: 'Freelance', value: 6000 },
//         ],
//       },
//       {
//         name: 'John',
//         age: 33,
//         email: 'john@example.com',
//         income: [
//           { name: 'Salary', value: 7000 },
//           { name: 'Stocks', value: 5000 },
//           { name: 'Investments', value: 8000 },
//         ],
//       },
      
//   ]);
//   const pieChartRef = useRef(null);
//   const barChartRef = useRef(null);

//   useEffect(() => {
//     if (pieChartRef.current && barChartRef.current) {
//       const user = users[activeSheet];
      
//       // create pie chart
//       const pieChartData = {
//         labels: user.income.map((item) => item.name),
//         datasets: [
//           {
//             data: user.income.map((item) => item.value),
//             backgroundColor: [
//               'rgba(255, 99, 132, 0.2)',
//               'rgba(54, 162, 235, 0.2)',
//               'rgba(255, 206, 86, 0.2)',
//               'rgba(23, 106, 44, 0.2)',
//             ],
//             borderColor: [
//               'rgba(255, 99, 132, 1)',
//               'rgba(54, 162, 235, 1)',
//               'rgba(255, 206, 86, 1)',
//               'rgba(23, 106, 44, 1)',
//             ],
//             borderWidth: 1,
//           },
//         ],
//       };
//       const pieChartInstance = new Chart(pieChartRef.current, {
//         type: 'pie',
//         data: pieChartData,
//         options: {},
//       });

//       // create bar chart
//       const barChartData = {
//         labels: user.income.map((item) => item.name),
//         datasets: [
//           {
//             label: user.name,
//             data: user.income.map((item) => item.value),
//             backgroundColor: [
//               'rgba(255, 99, 132, 0.2)',
//               'rgba(54, 162, 235, 0.2)',
//               'rgba(255, 206, 86, 0.2)',
//               'rgba(23, 206, 44, 0.2)',
//             ],
//             borderColor: [
//               'rgba(255, 99, 132, 1)',
//               'rgba(54, 162, 235, 1)',
//               'rgba(255, 206, 86, 1)',
//               'rgba(23, 106, 44, 1)',
//             ],
//             borderWidth: 1,
//           },
//         ],
//       };
//       const barChartInstance = new Chart(barChartRef.current, {
//         type: 'bar',
//         data: barChartData,
//         options: {},
//       });

//       return () => {
//         pieChartInstance.destroy();
//         barChartInstance.destroy();
//       };
//     }
//   }, [activeSheet, users]);

//   function handleTabClick(index) {
//     setActiveSheet(index);
// }

// return (
//   <><div className="row">
//     <a href="" target="_blank">
//       <img
//         src="/Logo_Ezhire.svg"
//         className="logo tauri"
//         alt="Tauri logo" />
//     </a>
//   </div><><div className="tabs-div">
//     {users.map((user, index) => (
//       <button
//         key={index}
//         className={`my-button ${activeSheet === index ? 'active' : ''}`}
//         onClick={() => handleTabClick(index)}
//       >
//         {user.name}
//       </button>

//     ))}
//   </div>

//   <div  style={{ height: "50px" }}>
//           <div style={{ width: "45%", float: "left" ,  display:"flex",marginLeft:"2.5%", marginTop:"10px"}}>
//           <p>Name: {users[activeSheet].name}</p>
//           <br></br>
//         <p>Age: {users[activeSheet].age}</p>
//         <br></br>
//         <p>Email: {users[activeSheet].email}</p></div>
//         <br></br>
//           <div className="charts-pie" style={{  float: "right" , display:"flex", marginRight:"2.5%", justifyContent:"flex-end"}}>
//           <canvas ref={pieChartRef} />   
//           </div>
//           <br></br>
//           <div className="charts-bar" style={{  float: "right" , display:"flex", marginRight:"2.5%", justifyContent:"flex-end"}}>
//           <canvas ref={barChartRef} />   
//           </div>
//         </div>
//       {/* <div className="user-info">
//         <p>Name: {users[activeSheet].name}</p>
//         <p>Age: {users[activeSheet].age}</p>
//         <p>Email: {users[activeSheet].email}</p>
//       </div>
//       <div className="charts-pie">
//         <canvas ref={pieChartRef} />
//       </div>
//       <div className="charts-bar">
//         <canvas ref={barChartRef} />
//       </div> */}
//     </></>
// );
// }

// export default Comparison;


// //10000percent only names//


// //greatest//
// // import React, { useState, useRef, useEffect } from 'react';
// // import Chart from 'chart.js/auto';
// // import './style.css';

// // function Comparison() {
// //   const [activeSheet, setActiveSheet] = useState(0);
// //   const [users, setUsers] = useState([
// //     {
// //       name: 'Alice',
// //       income: [
// //         { name: 'Salary', value: 5000 },
// //         { name: 'Bonus', value: 2000 },
// //         { name: 'Investments', value: 10000 },
// //       ],
// //     },
// //     {
// //       name: 'Bob',
// //       income: [
// //         { name: 'Salary', value: 7000 },
// //         { name: 'Bonus', value: 1500 },
// //         { name: 'Investments', value: 5000 },
// //       ],
// //     },
// //     {
// //       name: 'Charlie',
// //       income: [
// //         { name: 'Salary', value: 6000 },
// //         { name: 'Bonus', value: 3000 },
// //         { name: 'Investments', value: 8000 },
// //       ],
// //     },
// //   ]);

// //   const chartRef = useRef(null);

// //   useEffect(() => {
// //     if (chartRef.current) {
// //       const user = users[activeSheet];
// //       const data = {
// //         labels: user.income.map((item) => item.name),
// //         datasets: [
// //           {
// //             data: user.income.map((item) => item.value),
// //             backgroundColor: [
// //               'rgba(255, 99, 132, 0.2)',
// //               'rgba(54, 162, 235, 0.2)',
// //               'rgba(255, 206, 86, 0.2)',
// //             ],
// //             borderColor: [
// //               'rgba(255, 99, 132, 1)',
// //               'rgba(54, 162, 235, 1)',
// //               'rgba(255, 206, 86, 1)',
// //             ],
// //             borderWidth: 1,
// //           },
// //         ],
// //       };
// //       const chartInstance = new Chart(chartRef.current, {
// //         type: 'pie',
// //         data: data,
// //         options: {},
// //       });
// //       return () => {
// //         chartInstance.destroy();
// //       };
// //     }
// //   }, [activeSheet, users]);

// //   function handleTabClick(index) {
// //     setActiveSheet(index);
// //   }

// //   return (
// //     <>
// //       <div className="tabs-div">
// //         <div className="tabs">
// //           {users.map((user, index) => (
// //             <button
// //               key={index}
// //               className={activeSheet === index ? 'active' : ''}
// //               onClick={() => handleTabClick(index)}
// //             >
// //               {user.name}
// //             </button>
// //           ))}
// //         </div>
// //       </div>
// //       <div className="sheet">
// //         <h2>{users[activeSheet].name}</h2>
// //         <table>
// //           <thead>
// //             <tr>
// //               <th>Name</th>
// //               <th>Value</th>
// //             </tr>
// //           </thead>
// //             {users[activeSheet].income.map((item, index) => (
// //               <tr key={index}>
// //                 <th>{item.name}</th>
// //                 <th>{item.value}</th>
// //               </tr>
// //             ))}
          
// //         </table>
// //       </div>
// //       <div className="chart-container">
// //         <canvas ref={chartRef} />
// //       </div>
// //     </>
// //   );
// // }

// // export default Comparison;

