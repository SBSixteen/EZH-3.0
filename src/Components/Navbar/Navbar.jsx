// import React ,{useState} from "react";
// import "./Navbar.css";

// function Navbar(props) {   
//     const [ active, setActive] = useState('nav_menu');
//     const [toggleIcon, setToggleIcon] = useState("nav__toggler");
//     const navToggle = () =>{
//     active === 'nav__menu'
//     ? setActive('nav_menu nav__active')
//     : setActive("nav_menu");
// toggleIcon === "nav__toggler"
// ? setToggleIcon("nav__toggler toggle")
// : setToggleIcon("nav__toggler")

// }

//     return (
//     <nav className="nav">
//       <a href="#" className="nav__brand">
//         Muchachos
//       </a>
//       <ul className={active}>
//         <li className="nav_item">
//           <a href="#" className="nav__link"></a>Home
//         </li>
//         <li className="nav_item">
//           <a href="#" className="nav__link"></a>Dataset
//         </li>
//         <li className="nav_item">
//           <a href="#" className="nav__link"></a>Settings
//         </li>
//         <li className="nav_item">
//           <a href="#" className="nav__link"></a>Subscription
//         </li>
//       </ul>
//       <div onClick = {navToggle} className={toggleIcon}>
//         <div className="line1"></div>
//         <div className="line2"></div>
//         <div className="line3"></div>
//       </div>
//     </nav>
//   );
// }

// export default Navbar;

import React from 'react';
import './Navbar.css'; // Import the CSS file

const Navbar = () => {
  return (
    <nav className="navbar">
      <ul className="navbar-list">
        <li className="navbar-item">
          <a href="#" className="navbar-link">Home</a>
        </li>
        <li className="navbar-item">
          <a href="#" className="navbar-link">Subscriptions</a>
        </li>
        <li className="navbar-item">
          <a href="#" className="navbar-link">Settings</a>
        </li>
        <li className="navbar-item">
          <a href="#" className="navbar-link">LogOut</a>
        </li>
      </ul>
    </nav>
  );
};

export default Navbar;
