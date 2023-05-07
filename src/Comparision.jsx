import React, { useState, useRef, useEffect } from 'react';
import Chart from 'chart.js/auto';
import './style.css';

function Comparison() {
  const [activeSheet, setActiveSheet] = useState(0);
  const [users, setUsers] = useState([
    {
        name: 'Alice',
        age: 30,
        email: 'alice@example.com',
        income: [
          { name: 'Salary', value: 5000 },
          { name: 'B2B', value: 2000 },
          { name: 'Investments', value: 10000 },
        ],
      },
      {
        name: 'Bob',
        age: 35,
        email: 'bob@example.com',
        income: [
          { name: 'Salary', value: 7000 },
          { name: 'Bonus', value: 1500 },
          { name: 'Dividend', value: 5000 },
        ],
      },
      {
        name: 'Charlie',
        age: 25,
        email: 'charlie@example.com',
        income: [
          { name: 'abbakapaisa', value: 6000 },
          { name: 'stocks', value: 3000 },
          { name: 'jugaar', value: 8000 },
        ],
      },
      {
        name: 'David',
        age: 32,
        email: 'david@example.com',
        income: [
          { name: 'Salary', value: 6000 },
          { name: 'Bonus', value: 2500 },
          { name: 'Stocks', value: 9000 },
        ],
      },
      {
        name: 'Emily',
        age: 28,
        email: 'emily@example.com',
        income: [
          { name: 'Salary', value: 4000 },
          { name: 'Consulting', value: 3000 },
          { name: 'Freelance', value: 7000 },
        ],
      },
      {
        name: 'Frank',
        age: 40,
        email: 'frank@example.com',
        income: [
          { name: 'Salary', value: 8000 },
          { name: 'Stocks', value: 2000 },
          { name: 'Real Estate', value: 12000 },
        ],
      },
      {
        name: 'Grace',
        age: 27,
        email: 'grace@example.com',
        income: [
          { name: 'Salary', value: 4500 },
          { name: 'Consulting', value: 3500 },
          { name: 'Investments', value: 9000 },
        ],
      },
      {
        name: 'Harry',
        age: 31,
        email: 'harry@example.com',
        income: [
          { name: 'Salary', value: 6500 },
          { name: 'B2B', value: 1500 },
          { name: 'Stocks', value: 8000 },
        ],
      },
      {
        name: 'Isabella',
        age: 29,
        email: 'isabella@example.com',
        income: [
          { name: 'Salary', value: 5000 },
          { name: 'Bonus', value: 1000 },
          { name: 'Freelance', value: 6000 },
        ],
      },
      {
        name: 'John',
        age: 33,
        email: 'john@example.com',
        income: [
          { name: 'Salary', value: 7000 },
          { name: 'Stocks', value: 5000 },
          { name: 'Investments', value: 8000 },
        ],
      },{
        name: 'Alice',
        age: 30,
        email: 'alice@example.com',
        income: [
          { name: 'Salary', value: 5000 },
          { name: 'B2B', value: 2000 },
          { name: 'Investments', value: 10000 },
        ],
      },
      {
        name: 'Bob',
        age: 35,
        email: 'bob@example.com',
        income: [
          { name: 'Salary', value: 7000 },
          { name: 'Bonus', value: 1500 },
          { name: 'Dividend', value: 5000 },
        ],
      },
      {
        name: 'Charlie',
        age: 25,
        email: 'charlie@example.com',
        income: [
          { name: 'abbakapaisa', value: 6000 },
          { name: 'stocks', value: 3000 },
          { name: 'jugaar', value: 8000 },
        ],
      },
      {
        name: 'David',
        age: 32,
        email: 'david@example.com',
        income: [
          { name: 'Salary', value: 6000 },
          { name: 'Bonus', value: 2500 },
          { name: 'Stocks', value: 9000 },
        ],
      },
      {
        name: 'Emily',
        age: 28,
        email: 'emily@example.com',
        income: [
          { name: 'Salary', value: 4000 },
          { name: 'Consulting', value: 3000 },
          { name: 'Freelance', value: 7000 },
        ],
      },
      {
        name: 'Frank',
        age: 40,
        email: 'frank@example.com',
        income: [
          { name: 'Salary', value: 8000 },
          { name: 'Stocks', value: 2000 },
          { name: 'Real Estate', value: 12000 },
        ],
      },
      {
        name: 'Grace',
        age: 27,
        email: 'grace@example.com',
        income: [
          { name: 'Salary', value: 4500 },
          { name: 'Consulting', value: 3500 },
          { name: 'Investments', value: 9000 },
        ],
      },
      {
        name: 'Harry',
        age: 31,
        email: 'harry@example.com',
        income: [
          { name: 'Salary', value: 6500 },
          { name: 'B2B', value: 1500 },
          { name: 'Stocks', value: 8000 },
        ],
      },
      {
        name: 'Isabella',
        age: 29,
        email: 'isabella@example.com',
        income: [
          { name: 'Salary', value: 5000 },
          { name: 'Bonus', value: 1000 },
          { name: 'Freelance', value: 6000 },
        ],
      },
      {
        name: 'John',
        age: 33,
        email: 'john@example.com',
        income: [
          { name: 'Salary', value: 7000 },
          { name: 'Stocks', value: 5000 },
          { name: 'Investments', value: 8000 },
        ],
      },{
        name: 'Alice',
        age: 30,
        email: 'alice@example.com',
        income: [
          { name: 'Salary', value: 5000 },
          { name: 'B2B', value: 2000 },
          { name: 'Investments', value: 10000 },
        ],
      },
      {
        name: 'Bob',
        age: 35,
        email: 'bob@example.com',
        income: [
          { name: 'Salary', value: 7000 },
          { name: 'Bonus', value: 1500 },
          { name: 'Dividend', value: 5000 },
        ],
      },
      {
        name: 'Charlie',
        age: 25,
        email: 'charlie@example.com',
        income: [
          { name: 'abbakapaisa', value: 6000 },
          { name: 'stocks', value: 3000 },
          { name: 'jugaar', value: 8000 },
        ],
      },
      {
        name: 'David',
        age: 32,
        email: 'david@example.com',
        income: [
          { name: 'Salary', value: 6000 },
          { name: 'Bonus', value: 2500 },
          { name: 'Stocks', value: 9000 },
        ],
      },
      {
        name: 'Emily',
        age: 28,
        email: 'emily@example.com',
        income: [
          { name: 'Salary', value: 4000 },
          { name: 'Consulting', value: 3000 },
          { name: 'Freelance', value: 7000 },
        ],
      },
      {
        name: 'Frank',
        age: 40,
        email: 'frank@example.com',
        income: [
          { name: 'Salary', value: 8000 },
          { name: 'Stocks', value: 2000 },
          { name: 'Real Estate', value: 12000 },
        ],
      },
      {
        name: 'Grace',
        age: 27,
        email: 'grace@example.com',
        income: [
          { name: 'Salary', value: 4500 },
          { name: 'Consulting', value: 3500 },
          { name: 'Investments', value: 9000 },
        ],
      },
      {
        name: 'Harry',
        age: 31,
        email: 'harry@example.com',
        income: [
          { name: 'Salary', value: 6500 },
          { name: 'B2B', value: 1500 },
          { name: 'Stocks', value: 8000 },
        ],
      },
      {
        name: 'Isabella',
        age: 29,
        email: 'isabella@example.com',
        income: [
          { name: 'Salary', value: 5000 },
          { name: 'Bonus', value: 1000 },
          { name: 'Freelance', value: 6000 },
        ],
      },
      {
        name: 'John',
        age: 33,
        email: 'john@example.com',
        income: [
          { name: 'Salary', value: 7000 },
          { name: 'Stocks', value: 5000 },
          { name: 'Investments', value: 8000 },
        ],
      },
      
  ]);
  const pieChartRef = useRef(null);
  const barChartRef = useRef(null);

  useEffect(() => {
    if (pieChartRef.current && barChartRef.current) {
      const user = users[activeSheet];
      
      // create pie chart
      const pieChartData = {
        labels: user.income.map((item) => item.name),
        datasets: [
          {
            data: user.income.map((item) => item.value),
            backgroundColor: [
              'rgba(255, 99, 132, 0.2)',
              'rgba(54, 162, 235, 0.2)',
              'rgba(255, 206, 86, 0.2)',
            ],
            borderColor: [
              'rgba(255, 99, 132, 1)',
              'rgba(54, 162, 235, 1)',
              'rgba(255, 206, 86, 1)',
            ],
            borderWidth: 1,
          },
        ],
      };
      const pieChartInstance = new Chart(pieChartRef.current, {
        type: 'pie',
        data: pieChartData,
        options: {},
      });

      // create bar chart
      const barChartData = {
        labels: user.income.map((item) => item.name),
        datasets: [
          {
            label: user.name,
            data: user.income.map((item) => item.value),
            backgroundColor: [
              'rgba(255, 99, 132, 0.2)',
              'rgba(54, 162, 235, 0.2)',
              'rgba(255, 206, 86, 0.2)',
            ],
            borderColor: [
              'rgba(255, 99, 132, 1)',
              'rgba(54, 162, 235, 1)',
              'rgba(255, 206, 86, 1)',
            ],
            borderWidth: 1,
          },
        ],
      };
      const barChartInstance = new Chart(barChartRef.current, {
        type: 'bar',
        data: barChartData,
        options: {},
      });

      return () => {
        pieChartInstance.destroy();
        barChartInstance.destroy();
      };
    }
  }, [activeSheet, users]);

  function handleTabClick(index) {
    setActiveSheet(index);
}

return (
        <><div className="tabs-div">
        {users.map((user, index) => (
          <button
            key={index}
            className={activeSheet === index ? 'active' : ''}
            onClick={() => handleTabClick(index)}
          >
            {user.name}
          </button>
        ))}
      </div>
      <div className="user-info">
        <p>Name: {users[activeSheet].name}</p>
        <p>Age: {users[activeSheet].age}</p>
        <p>Email: {users[activeSheet].email}</p>
      </div>
      <div className="charts-pie">
        <canvas ref={pieChartRef} />
      </div>
      <div className="charts-bar">
        <canvas ref={barChartRef} />
      </div>
      </>
);
}

export default Comparison;


//10000percent only names//


//greatest//
// import React, { useState, useRef, useEffect } from 'react';
// import Chart from 'chart.js/auto';
// import './style.css';

// function Comparison() {
//   const [activeSheet, setActiveSheet] = useState(0);
//   const [users, setUsers] = useState([
//     {
//       name: 'Alice',
//       income: [
//         { name: 'Salary', value: 5000 },
//         { name: 'Bonus', value: 2000 },
//         { name: 'Investments', value: 10000 },
//       ],
//     },
//     {
//       name: 'Bob',
//       income: [
//         { name: 'Salary', value: 7000 },
//         { name: 'Bonus', value: 1500 },
//         { name: 'Investments', value: 5000 },
//       ],
//     },
//     {
//       name: 'Charlie',
//       income: [
//         { name: 'Salary', value: 6000 },
//         { name: 'Bonus', value: 3000 },
//         { name: 'Investments', value: 8000 },
//       ],
//     },
//   ]);

//   const chartRef = useRef(null);

//   useEffect(() => {
//     if (chartRef.current) {
//       const user = users[activeSheet];
//       const data = {
//         labels: user.income.map((item) => item.name),
//         datasets: [
//           {
//             data: user.income.map((item) => item.value),
//             backgroundColor: [
//               'rgba(255, 99, 132, 0.2)',
//               'rgba(54, 162, 235, 0.2)',
//               'rgba(255, 206, 86, 0.2)',
//             ],
//             borderColor: [
//               'rgba(255, 99, 132, 1)',
//               'rgba(54, 162, 235, 1)',
//               'rgba(255, 206, 86, 1)',
//             ],
//             borderWidth: 1,
//           },
//         ],
//       };
//       const chartInstance = new Chart(chartRef.current, {
//         type: 'pie',
//         data: data,
//         options: {},
//       });
//       return () => {
//         chartInstance.destroy();
//       };
//     }
//   }, [activeSheet, users]);

//   function handleTabClick(index) {
//     setActiveSheet(index);
//   }

//   return (
//     <>
//       <div className="tabs-div">
//         <div className="tabs">
//           {users.map((user, index) => (
//             <button
//               key={index}
//               className={activeSheet === index ? 'active' : ''}
//               onClick={() => handleTabClick(index)}
//             >
//               {user.name}
//             </button>
//           ))}
//         </div>
//       </div>
//       <div className="sheet">
//         <h2>{users[activeSheet].name}</h2>
//         <table>
//           <thead>
//             <tr>
//               <th>Name</th>
//               <th>Value</th>
//             </tr>
//           </thead>
//             {users[activeSheet].income.map((item, index) => (
//               <tr key={index}>
//                 <th>{item.name}</th>
//                 <th>{item.value}</th>
//               </tr>
//             ))}
          
//         </table>
//       </div>
//       <div className="chart-container">
//         <canvas ref={chartRef} />
//       </div>
//     </>
//   );
// }

// export default Comparison;

//greatest//


// import React, { useState, useRef, useEffect } from 'react';
// import Chart from 'chart.js/auto';
// import "./style.css";

// function Comparison() {
//   const userData = [    {      name: 'Alice',      age: 1,      email: 'a',      income: [        { name: 'Salary', value: 5000 },        { name: 'Investments', value: 2000 },        { name: 'Rent', value: 1000 }      ]
//     },
//     {
//       name: 'Bob',
//       age: 30,
//       email: 'bob@example.com',
//       income: [
//         { name: 'Salary', value: 6000 },
//         { name: 'Investments', value: 3000 },
//         { name: 'Rent', value: 2000 }
//       ]
//     },
//     {
//       name: 'Charlie',
//       age: 35,
//       email: 'charlie@example.com',
//       income: [
//         { name: 'Salary', value: 4000 },
//         { name: 'Investments', value: 1000 },
//         { name: 'Rent', value: 1500 }
//       ]
//     },
//   ];

//   const [activeSheet, setActiveSheet] = useState(1);
//   const [activeIncome, setActiveIncome] = useState(userData[0].income[0].name);
//   const [chartData, setChartData] = useState(null);
//   const chartRef = useRef(null);

//   useEffect(() => {
//     const incomeData = userData[activeSheet - 1].income.find(income => income.name === activeIncome);
//     const data = {
//       labels: userData[activeSheet - 1].income.map(income => income.name),
//       datasets: [
//         {
//           data: userData[activeSheet - 1].income.map(income => income.value),
//           backgroundColor: [
//             'rgba(255, 99, 132, 0.2)',
//             'rgba(54, 162, 235, 0.2)',
//             'rgba(255, 206, 86, 0.2)',
//           ],
//           borderColor: [
//             'rgba(255, 99, 132, 1)',
//             'rgba(54, 162, 235, 1)',
//             'rgba(255, 206, 86, 1)',
//           ],
//           borderWidth: 1,
//         },
//       ],
//     };
//     setChartData(data);
//   }, [activeSheet, activeIncome]);

//   function handleTabClick(sheetNumber) {
//     setActiveSheet(sheetNumber);
//     setActiveIncome(userData[sheetNumber - 1].income[0].name);
//   }

//   function handleIncomeClick(incomeName) {
//     setActiveIncome(incomeName);
//   }

//   useEffect(() => {
//     if (chartRef.current && chartData) {
//       const chartInstance = new Chart(chartRef.current, {
//         type: 'pie',
//         data: chartData,
//         options: {},
//       });
//       return () => {
//         chartInstance.destroy();
//       };
//     }
//   }, [chartData]);

//   return (
//     <>
//       <div className="tabs-div">
//         <div className="tabs">
//           {userData.map((user, index) => (
//             <button
//               key={index}
//               className={activeSheet === index + 1 ? 'active' : ''}
//               onClick={() => handleTabClick(index + 1)}
//             >
//               {user.name}
//             </button>
//           ))}
//         </div>
//         <div className="income-tabs">
//           {userData[activeSheet - 1].income.map((income, index) => (
//                     <button
//                     key={index}
//                     className={activeIncome === income.name ? 'active' : ''}
//                     onClick={() => handleIncomeClick(income.name)}
//                   >
//                     {income.name}
//                   </button>
//                 ))}
//               </div>
//             </div>
//             <div className="chart-container">
//               <canvas ref={chartRef} />
//             </div>
//           </>
// );
// }

// export default Comparison;          


/*gooood one*/
// import React, { useState, useRef, useEffect } from 'react';
// import Chart from 'chart.js/auto';
// import "./style.css";

// function Comparison() {
//   const userData = [
//     {
//       name: 'Alice',
//       age: 1,
//       email: 'a',
//     },
//     {
//       name: 'Bob',
//       age: 30,
//       email: 'bob@example.com',
//     },
//     {
//       name: 'Charlie',
//       age: 35,
//       email: 'charlie@example.com',
//     },
//   ];

//   const [activeSheet, setActiveSheet] = useState(1);
//   const [chartData, setChartData] = useState({
//     labels: ['Age', 'Email Length'],
//     datasets: [
//       {
//         data: [25, 16],
//         backgroundColor: [
//           'rgba(255, 99, 132, 0.2)',
//           'rgba(54, 162, 235, 0.2)',
//         ],
//         borderColor: [
//           'rgba(255, 99, 132, 1)',
//           'rgba(54, 162, 235, 1)',
//         ],
//         borderWidth: 1,
//       },
//     ],
//   });

//   const chartRef = useRef(null);

//   useEffect(() => {
//     if (chartRef.current && chartData.datasets) {
//       const chartInstance = new Chart(chartRef.current, {
//         type: 'pie',
//         data: chartData,
//         options: {},
//       });
//       return () => {
//         chartInstance.destroy();
//       };
//     }
//   }, [chartData]);

//   function handleTabClick(sheetNumber) {
//     setActiveSheet(sheetNumber);

//     const data = {
//       labels: ['Age', 'Email Length'],
//       datasets: [
//         {
//           data: [userData[sheetNumber - 1].age, userData[sheetNumber - 1].email.length],
//           backgroundColor: [
//             'rgba(255, 99, 132, 0.2)',
//             'rgba(54, 162, 235, 0.2)',
//           ],
//           borderColor: [
//             'rgba(255, 99, 132, 1)',
//             'rgba(54, 162, 235, 1)',
//           ],
//           borderWidth: 1,
//         },
//       ],
//     };

//     setChartData(data);
//   }

//   return (
//     <>
//       <div className="tabs-div">
//         <div className="tabs">
//           {userData.map((user, index) => (
//             <button
//               key={index}
//               className={activeSheet === index + 1 ? 'active' : ''}
//               onClick={() => handleTabClick(index + 1)}
//             >
//               {user.name}
//             </button>
//           ))}
//         </div>
//       </div>
//       <div className="sheet">
//         <h2>{userData[activeSheet - 1].name}</h2>
//         <table>
//           <thead>
//             <tr>
//               <th>Name</th>
//               <th>Age</th>
//               <th>Email</th>
//             </tr>
//           </thead>

//           <tbody>
//             <tr>
//               <th>{userData[activeSheet - 1].name}</th>
//               <th>{userData[activeSheet - 1].age}</th>
//               <th>{userData[activeSheet - 1].email}</th>
//             </tr>
//           </tbody>
//         </table>
//       </div>
//       <div className="chart-container">
//         <canvas ref={chartRef} />
//       </div>
//     </>
//   );
// }

// export default Comparison;

/*gooooododododod one*/

// import React, { useState } from 'react';
// import Chart from 'chart.js/auto';
// import "./style.css";

// function Comparison() {
//   const userData = [
//     {
//       name: 'Alice',
//       age: 25,
//       email: 'alice@example.com',
//     },
//     {
//       name: 'Bob',
//       age: 30,
//       email: 'bob@example.com',
//     },
//     {
//       name: 'Charlie',
//       age: 35,
//       email: 'charlie@example.com',
//     },
//   ];

//   const [activeSheet, setActiveSheet] = useState(1);
//   const [chartData, setChartData] = useState({
//     labels: ['Age', 'Email Length'],
//     datasets: [
//       {
//         data: [25, 16],
//         backgroundColor: [
//           'rgba(255, 99, 132, 0.2)',
//           'rgba(54, 162, 235, 0.2)',
//         ],
//         borderColor: [
//           'rgba(255, 99, 132, 1)',
//           'rgba(54, 162, 235, 1)',
//         ],
//         borderWidth: 1,
//       },
//     ],
//   });

//   function handleTabClick(sheetNumber) {
//     setActiveSheet(sheetNumber);

//     const data = {
//       labels: ['Age', 'Email Length'],
//       datasets: [
//         {
//           data: [userData[sheetNumber - 1].age, userData[sheetNumber - 1].email.length],
//           backgroundColor: [
//             'rgba(255, 99, 132, 0.2)',
//             'rgba(54, 162, 235, 0.2)',
//           ],
//           borderColor: [
//             'rgba(255, 99, 132, 1)',
//             'rgba(54, 162, 235, 1)',
//           ],
//           borderWidth: 1,
//         },
//       ],
//     };

//     setChartData(data);
//   }

//   return (
//     <>
//       <div className="tabs-div">
//         <div className="tabs">
//           {userData.map((user, index) => (
//             <button
//               key={index}
//               className={activeSheet === index + 1 ? 'active' : ''}
//               onClick={() => handleTabClick(index + 1)}
//             >
//               {user.name}
//             </button>
//           ))}
//         </div>
//       </div>
//       <div className="sheet">
//         <h2>{userData[activeSheet - 1].name}</h2>
//         <table>
//           <thead>
//             <tr>
//               <th>Name</th>
//               <th>Age</th>
//               <th>Email</th>
//             </tr>
//           </thead>

//           <tbody>
//             <tr>
//               <th>{userData[activeSheet - 1].name}</th>
//               <th>{userData[activeSheet - 1].age}</th>
//               <th>{userData[activeSheet - 1].email}</th>
//             </tr>
//           </tbody>
//         </table>
//         <div style={{ width: '400px', height: '400px' }}>
//           <canvas ref={(ref) => {
//             if (ref && chartData.datasets) {
//               new Chart(ref, {
//                 type: 'pie',
//                 data: chartData,
//                 options: {},
//               });
//             }
//           }} />
//         </div>
//       </div>
//     </>
//   );
// }

// export default Comparison;
