import React from 'react';

const Header = ({ category, title }) => (
  <div style={{
    paddingLeft: '100px',
    paddingTop: '100px',
    boxSizing: 'content-box',
  }}>
    <h1>
      {title}
      </h1>
  </div>
);

export default Header;
