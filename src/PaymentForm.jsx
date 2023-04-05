import React, { useState } from "react";
import Stripe from "stripe";
import Button from "./Components/Button";

const stripe = new Stripe("your_stripe_api_key_here");

function PaymentForm() {
  const [cardNumber, setCardNumber] = useState("");
  const [expiryMonth, setExpiryMonth] = useState("");
  const [expiryYear, setExpiryYear] = useState("");
  const [cvc, setCvc] = useState("");
  const [accountHolder, setAccountHolder] = useState("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    const cardElement = stripe.elements().create("card");
    const { token } = await stripe.createToken(cardElement, {
      card: {
        name: accountHolder,
        number: cardNumber,
        exp_month: expiryMonth,
        exp_year: expiryYear,
        cvc: cvc,
      },
    });
    console.log(token.id);
    // Do something with the token, such as submitting it to your server for payment processing
  };

  return (
    <form onSubmit={handleSubmit}>
      <label>
        <h1>Payment Gateway</h1>
      </label>
      <br></br>
      <label>
        Account Holder Name
        <br></br>
        <input
          type="text"
          value={cardNumber}
          onChange={(e) => setAccountHolder(e.target.value)}
          placeholder="Name"
        />
      </label>
      <br></br>
      <label>
        Card number
        <br></br>
        <input
          type="text"
          value={cardNumber}
          onChange={(e) => setCardNumber(e.target.value)}
          placeholder="XXXX XXXX XXXX XXXX"
        />
      </label>

      <br></br>

      <label>
        Expiry
        <br></br>
        <input
          type="text"
          value={expiryYear}
          onChange={(e) => setExpiryYear(e.target.value)}
          placeholder="MM/YY"
        />
      </label>

      <br></br>

      <label>
        CVC
        <br></br>
        <input
          type="password"
          id="pwd"
          name="pwd"
          minlength="8"
          value={cvc}
          onChange={(e) => setCvc(e.target.value)}
          placeholder="* * *"
        />
      </label>
      <br></br>

      <Button title="Send Payment" />
    </form>
  );
}

export default PaymentForm;
