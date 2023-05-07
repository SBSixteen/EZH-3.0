import './Startingpage.css';
import { useNavigate } from "react-router-dom";

const Startingpage = () => {

    let navigate = useNavigate();
    const gotoRegister = () => {
        let path = "/Register";
        navigate(path);
    };

    const gotoLogin = () => {
        let path = "/login";
        navigate(path);
      };

    return(
        <div className='staringpage'>
            <div className="startingh2">
                Welcome to EZ Hire
            </div>
            <div className="StyledSubTitle">
                Hiring made simpler
            </div>
            <div className="button-group">
                <button className="StyledButton" onClick={() => {
                gotoLogin();
              }}>Login</button>
                <button className="StyledButton" onClick={() => {
                gotoRegister();
              }}>SignUp</button>
            </div>
        </div>
    );
}

export default Startingpage;