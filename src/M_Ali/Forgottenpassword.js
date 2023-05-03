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

const Forgotpass = () => {
    return(
        <div>
            <StyledFormArea>
                <StyledTitle color={colors.dark1} size={38}>
                    Password Reset
                </StyledTitle>
                <Formik
                    initialValues = {{
                        email:"",
                        redirectUrl: "http://localhost:3000/forgottenpassword",
                    }}
                    validationSchema={Yup.object({
                            email: Yup.string()
                                .email("Invalid email address")
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
                                label="Enter your Email Address"
                                placeholder = "abc@gmail.com"
                                icon={<FiMail/>}
                            />

                            <ButtonGroup>
                                {!isSubmitting && <StyledFormButton type = "submit">Submit</StyledFormButton>}
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
            </StyledFormArea>
        </div>
    )
}

export default Forgotpass;