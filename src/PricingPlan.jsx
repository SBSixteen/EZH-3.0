import App from "./App.js";
import "./style.css";
import { Container } from "@mantine/core";
import React from "react";
import Sidebar from "./Sidebar.jsx";
import "./Home.css";

const PricingPlan = () => {
  const plans = [
    {
      title: "Basic Plan",
      price: "$10/month",
      features: ["1 GB Storage", "10 Users", "Email Support"],
    },
    {
      title: "Pro Plan",
      price: "$20/month",
      features: ["10 GB Storage", "100 Users", "Email + Phone Support"],
    },
    {
      title: "Enterprise Plan",
      price: "$50/month",
      features: [
        "100 GB Storage",
        "Unlimited Users",
        "24/7 Support ",
        "New element",
      ],
    },
  ];

  const PlanCard = ({ title, price, features }) => {
    return (
      <div>
        <Container height={700} width={300} p="md"></Container>

        <div className="plan-card">
          <h2>{title}</h2>
          <p className="price">{price}</p>
          <ul className="features">
            {features.map((feature, index) => (
              <li key={index}>{feature}</li>
            ))}
          </ul>
          <button>Select Plan</button>
        </div>
      </div>
    );
  };

  return (
    <>
      <div className="container-new">
        <div className="sidebar-flex">
          <Sidebar />
        </div>
        <div className="container-right-side"> 
          <div className="price-plan">
            {plans.map((plan, index) => (
              <PlanCard key={index} {...plan} />
            ))}
          </div>
        </div>
      </div>
    </>
  );
};

export default PricingPlan;
