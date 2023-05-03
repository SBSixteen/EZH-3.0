//styled components

import {StyledTextInput, StyledFormArea, StyledFormButton, StyledLabel, StyledTitle, Avatar, colors, ButtonGroup, ExtraText, TextLink} from './../components/Styles';
import Logo_Ezhire from "./../assets/Logo_Ezhire.png";

//formik
import { Formik, Form } from 'formik';
import { TextInput } from '../components/FormLib';
import * as Yup from 'yup';

//icons
import {FiMail, FiLock} from 'react-icons/fi';

//loader
import * as Loader from 'react-loader-spinner';

const Login = () => {
    return(
        <div>
            <StyledFormArea>
                <StyledTitle color={colors.dark1} size={38}>
                    Member Login
                </StyledTitle>
                <Formik
                    initialValues = {{
                        email:"",
                        password:"",
                    }}
                    validationSchema={Yup.object({
                            email: Yup.string()
                                .email("Invalid email address")
                                .required("Required"),
                            password: Yup.string()
                                .min(8, "Password is too short")
                                .max(30, "Password is too long")
                                .required("Required"),
                    })}
                    onSubmit={(values, {setSubmitting}) => {
                        console.log(values);
                    }}
                >
                    
                    {({isSubmitting}) => (
                        <Form>
                            <TextInput
                                name= "email"
                                type="text"
                                label="Email Address"
                                placeholder = "abc@gmail.com"
                                icon={<FiMail/>}
                            />

                            <TextInput
                                name= "password"
                                type="password"
                                label="Password"
                                placeholder = " ******** "
                                icon={<FiLock/>}
                            />

                            <ButtonGroup>
                                {!isSubmitting && <StyledFormButton type = "submit">Login</StyledFormButton>}
                                {isSubmitting && (
                                    <Loader
                                        type = "ThreeDots"
                                        color = {colors.theme}
                                        height = {49}
                                        width = {100}
                                    />
                                )}
                            </ButtonGroup>
                        </Form>
                    )}
                </Formik>
                <ExtraText>
                    Forgot Password? <TextLink to="/forgottenpassword">Reset it</TextLink>
                </ExtraText>
                <ExtraText>
                    New here? <TextLink to="/signup">Signup</TextLink>
                </ExtraText>
            </StyledFormArea>
        </div>
    )
}

export default Login;