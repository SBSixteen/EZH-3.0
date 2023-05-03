//styled components

import {StyledTextInput, StyledFormArea, StyledFormButton, StyledLabel, StyledTitle, Avatar, colors, ButtonGroup, ExtraText, TextLink} from './../components/Styles';
import Logo_Ezhire from "./../assets/Logo_Ezhire.png";

//formik
import { Formik, Form } from 'formik';
import { TextInput } from '../components/FormLib';
import * as Yup from 'yup';

//icons
import {FiMail, FiLock, FiUser, FiCalendar} from 'react-icons/fi';

//loader
import * as Loader from 'react-loader-spinner';

const Signup = () => {
    return(
        <div>
            <StyledFormArea>
                <StyledTitle color={colors.dark1} size={38}>
                    SignUp
                </StyledTitle>
                <Formik
                    initialValues = {{
                        email:"",
                        password:"",
                        repeatPassword:"",
                        dateOfBirth: "",
                        name: "",
                    }}
                    validationSchema={Yup.object({
                            email: Yup.string()
                                .email("Invalid email address")
                                .required("Required"),
                            password: Yup.string()
                                .min(8, "Password is too short")
                                .max(30, "Password is too long")
                                .required("Required"),
                            name: Yup.string().required("Required"),
                            dateOfBirth: Yup.date().required("Required"),
                            repeatPassword: Yup.string().required("Required").oneOf([Yup.ref("password")], "Password must match")
                    })}
                    onSubmit={(values, {setSubmitting}) => {
                        console.log(values);
                    }}
                >
                    
                    {({isSubmitting}) => (
                        <Form>
                            <TextInput
                                name= "name"
                                type="text"
                                label="Full Name"
                                placeholder = "Sherlock Holmes"
                                icon={<FiUser />}
                            />

                            <TextInput
                                name= "email"
                                type="text"
                                label="Email Address"
                                placeholder = "abc@gmail.com"
                                icon={<FiMail />}
                            />

                            <TextInput
                                name= "dateOfBirth"
                                type="datetime-local"
                                label="Date of Birth"
                                icon={<FiCalendar />}
                            />

                            <TextInput
                                name= "password"
                                type="password"
                                label="Password"
                                placeholder = " ******** "
                                icon={<FiLock/>}
                            />

                            <TextInput
                                name= "password"
                                type="password"
                                label="Repeat Password"
                                placeholder = " ******** "
                                icon={<FiLock/>}
                            />

                            <ButtonGroup>
                                {!isSubmitting && <StyledFormButton type = "submit">Signup</StyledFormButton>}
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
                    Already a user? <TextLink to="/login">Login</TextLink>
                </ExtraText>
            </StyledFormArea>
        </div>
    )
}

export default Signup;