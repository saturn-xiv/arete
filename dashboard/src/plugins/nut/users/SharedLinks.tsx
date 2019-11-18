import React from "react";
import { Link } from "react-router-dom";

const Widget: React.FC = () => {
  return (
    <nav>
      <ul>
        <li>
          <Link to="/">Home</Link>
        </li>
        <li>
          <Link to="/install">Install</Link>
        </li>
        <li>
          <Link to="/users/sign-in">Sign In </Link>
        </li>
        <li>
          <Link to="/users/sign-up">Sign Up</Link>
        </li>
      </ul>
    </nav>
  );
};

export default Widget;
