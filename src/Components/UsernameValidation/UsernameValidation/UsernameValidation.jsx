import React, { useState } from "react";

const UsernameForm = () => {
  const [username, setUsername] = useState("");
  const [error, setError] = useState("");

  const handleUsernameChange = (event) => {
    const { value } = event.target;
    setUsername(value);

    // validate username with regex
    const regex = /^[a-zA-Z0-9]{4,20}$/;
    if (regex.test(value)) {
      setError("");
    } else {
      setError("Username must be between 4 and 20 characters.");
    }
  };

  return (
    <div>
      <label htmlFor="username">Username:</label>
      <input
        id="username"
        type="text"
        value={username}
        onChange={handleUsernameChange}
      />
      {error && <div style={{ color: "red" }}>{error}</div>}
    </div>
  );
};

export default UsernameForm;
