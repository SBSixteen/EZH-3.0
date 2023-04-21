import React, { useState } from 'react';

const EmailForm = () => {
  const [email, setEmail] = useState('');
  const [error, setError] = useState('');

  const handleEmailChange = (event) => {
    const { value } = event.target;
    setEmail(value);

    // validate email with regex
    const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (regex.test(value)) {
      setError('');
    } else {
      setError('Please enter an Email address.');
    }
  };

  return (
    <div>
      <label htmlFor="email">Email:</label>
      <input
        id="email"
        type="email"
        value={email}
        onChange={handleEmailChange}
      />
      {error && <div style={{ color: 'red' }}>{error}</div>}
    </div>
  );
};

export default EmailForm;
