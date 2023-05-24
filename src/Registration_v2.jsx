import React, { useState, useEffect } from 'react'
import './v2.css'
import { invoke } from "@tauri-apps/api/tauri";

function Registration_v2() {

    const [email, setEmail] = useState("");
    const [name, setName] = useState("");
    const [password, setPassword] = useState("");
    const [cpassword, setCpassword] = useState("");
    const [check_e, setcheckE] = useState(false);
    const [check_p, setcheckP] = useState(false);
    const [check_2p, setcheck2P] = useState(false);
    const [check_n, setcheckN] = useState(false);
    const [transit, setTransit] = useState(true);
    const [helptext, sethelptext] = useState("");

    const emailValidation = () => {
        const regEx = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (regEx.test(email)) {
            setcheckE(true);
        } else if (!regEx.test(email)) {
            setcheckE(false);
        } else if (email.length == 0) {
            setcheckE(false);
        }
    };

    const passValidation = () => {
        if (password.length < 7) {
            setcheckP(false);
        } else {
            setcheckP(true);
        }
    };

    const twopass = () => {



        if (password == cpassword) {
            setcheck2P(true);
        } else {
            setcheck2P(false);
        }


    }

    const nameValidation = () => {

        if (name.length > 0) {
            setcheckN(true);
        } else {
            setcheckN(false);
        }

    }

    const validate = () => {
        emailValidation()
        passValidation()
        nameValidation()
        twopass()

        sethelptext("");

        var x = "";

        if (!check_e) {
            x = "Email is invalid."
        }

        if (!check_n) {
            x = x + " Name is empty."
        }

        if (!check_p) {
            x = x + " Password is too short." ;
        }

        if (!check_2p) {
            x = x + " Passwords do not match."
        }

        sethelptext(x);

        if (check_p && check_e && check_2p && check_n) {
            sethelptext("");
        }
    }

    // useEffect(()=>{
    //     const interval = setInterval(() =>validate(),1000);
    //     return()=>{
    //         clearInterval(interval);
    //     };
    // }, [])

    return (

        <div>

            <div>
                {/* {Someone get the logo loaded!!} */}
                <img src="/EZH Logo.svg" />
            </div>


            <title>EZHIRE</title>
            <div className='box' style={{ height: "650px" }}>

                <form>

                    <div className='inputBox'>

                        <h2>Sign In</h2>
                        <input type="text" required

                            onChange={(e) => {

                                setEmail(e.currentTarget.value)
                                validate();
                                validate();
                            }}

                        ></input>
                        <span>Email</span>
                        <i />
                    </div>

                    <div className='inputBox' style={{ marginTop: "0px" }} >
                        <input type="text" required 
                        onChange={(e) => {

                            setName(e.currentTarget.value)
                            validate();
                            validate();
                        }}></input>
                        <span>Full Name</span>
                        <i />
                    </div>

                    <div className='inputBox' >

                        <input type="password" style={{ marginTop: "-25px" }} required 
                        
                        onChange={(e) => {

                            setPassword(e.currentTarget.value)
                            validate();                                validate();
                        }
                    }
                        ></input>
                    <span>Enter Password</span>

                    <i />
            </div>
            <div className='inputBox' >

                <input type="password" style={{ marginTop: "-25px" }} required
                
                onChange={(e) => {

                    setCpassword(e.currentTarget.value)
                    validate();                                validate();
                }
            }
                ></input>
                <span>Confirm Password</span>

                <i />
            </div>
            <div onClick={(e) => {

                sethelptext("");
                validate();

                console.log(check_e);
                console.log(check_p);
                console.log(check_n);
                console.log(check_2p);

                if (check_e && check_p && check_n && check_2p && transit) {

                    setTransit(false);
                    sethelptext("Querying...")

                    invoke("create_user", {
                        email: email,
                        name: name,
                        password: password,
                    }).then(
                        (message) => {
                            var x = JSON.parse(message);
                            console.log(x);
                            sethelptext(x.response);

                            if (x.value) {
                                sessionStorage.setItem("acc", email);
                            }

                            setTransit(true);
                        }
                    )
                    console.log("end of task");

                }
            }
            }>

                <a className='LogRegButt'> Login </a>

            </div>
            <div className='links' style={{ marginTop: "-45px" }}>
                {/* Temporary ? for ease of access */}
                <a href="/">?</a>
                <a href="/logv2">Already have an account?</a>
                <a href="/">?</a>
            </div>

            {/* Add Login Implementation */}
            <a style={{ color: "#FFFFFF", marginTop: "-15px" }}> {helptext} </a>
        </form>
                {/* Need to integrate registration properly */ }



            </div >


        </div >

    );

}

export default Registration_v2;