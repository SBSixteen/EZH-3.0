import { StyledSubTitle, StyledTitle, Avatar, StyledButton, ButtonGroup} from "../components/Styles";

//Logo
import Logo_Ezhire from "./../assets/Logo_Ezhire.png";

const Home = () => {
    return(
        <div>
            <div>
            </div>
            <StyledTitle size={65}>
                Welcome to EZ Hire
            </StyledTitle>
            <StyledSubTitle size={27}>
                Hiring made simpler
            </StyledSubTitle>
            <ButtonGroup>
                <StyledButton to="/login">
                    Login
                </StyledButton>
                <StyledButton to="/signup">
                    SignUp
                </StyledButton>
            </ButtonGroup>
        </div>
    );
}

export default Home;