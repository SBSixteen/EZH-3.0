import React, { useState } from "react";
import StripeCheckout from "react-stripe-checkout";
import { Container, Typography, TextField } from "@material-ui/core";
import { makeStyles } from "@material-ui/core/styles";
import Button from "./Components/Button";

const useStyles = makeStyles((theme) => ({
  container: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    paddingTop: theme.spacing(4),
    paddingBottom: theme.spacing(4),
  },
  title: {
    marginBottom: theme.spacing(2),
  },
  input: {
    marginBottom: theme.spacing(2),
  },
  button: {
    marginTop: theme.spacing(2),
  },
}));

function PaymentPage() {
  const [product] = useState({
    name: "Final Payment",
    price: 19.99,
  });
  const classes = useStyles();

  const handleToken = (token) => {
    // Send the token to your backend to process the payment
    
  };

  return (
    <Container maxWidth="md" className={classes.container}>
      
      <h1>Check Out</h1>  
      <br></br>
    
      <Typography variant="h6">
        {product.name} - ${product.price}
      </Typography>
      <TextField label="Name" variant="outlined" className={classes.input} />
      
      <StripeCheckout
        stripeKey="<your_stripe_public_key>"
        token={handleToken}
        amount={product.price * 100}
        name={product.name}
      >
        <Button variant="contained"  className={classes.button}
          title = "Pay Now" >
        </Button>
      </StripeCheckout>
    </Container>
  );
}

export default PaymentPage;
