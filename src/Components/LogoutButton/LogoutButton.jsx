import React from 'react';
import { useHistory } from 'react-router-dom';

const LogoutButton = () => {
  const history = useHistory();

  const handleLogout = () => {
    // to clear any stored authentication information
    localStorage.removeItem('accessToken');
    // to navigate to login page or homepage
    history.push('/login');
  };

  return <button onClick={handleLogout}>Logout</button>;
};

export default LogoutButton;