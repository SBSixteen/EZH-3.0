import React, { useState } from 'react'
import './v2.css'
import { invoke } from "@tauri-apps/api/tauri";

function PassReset_v2() {

    const [transit, setTransit] = useState(true);
    const [verified, setVerified] = useState(false); 
    const [input, setInput] = useState("");
    const [helptext, sethelptext] = useState("");
    const [proceed, setProceed] = useState(false);
    const [proceedF, setProceedF] = useState(false);
    const [password, setPassword] = useState("");
    const [vcode, setVcode] = useState("");

    const emailValidation = () => {
        const regEx = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (regEx.test(input)) {
            setVerified(true);
        } else if (!regEx.test(input)) {
            setVerified(false);
        } else if (input.length == 0) {
            setVerified(false);
        }

        console.log(verified);
    };


    return (

        <div>
            <div className='box' style={{ height: "650px" }}>
                <form id = "login">
                    <div className='inputBox'>
                        <h2 style={{ fontSize: "22px", marginTop: "-35px" }}>Forgot Password</h2>
                        <input type="text" required style={{ marginTop: "-10px" }} onInput={e => setInput(e.target.value)}></input>
                        <span>Email</span>
                        <i />
                    </div>

                    <div onClick={(e) => {
                        
                        sethelptext("Querying...");
                        emailValidation();
                        
                        if(verified && transit){
                            setTransit(false);
                            invoke("generate_changepass_code", {
                                email: input,
                            }).then((message) => {
                                var x = JSON.parse(message);
                                sethelptext(x.response);
                                setProceed(x.value);
                                console.log(x);
                                setTransit(true);
                                if(proceed){
                                    localStorage.setItem("wadup", input);
                                }
                            })
                        }else{
                            sethelptext("Invalid Email!");
                        }
}                       
                    }>

                        <a className='LogRegButt'> Request Code </a>
                        
                    </div>

                    <div className='inputBox' style={{marginTop:"-15px"}}>
                        <input type="text" required style={{ marginTop: "-40px" }} id='2FACODE' onInput={e => setVcode(e.target.value)}></input>
                        <span>Verification Code</span>
                        <i />
                    </div>

                    <div onClick={(e) => {
                        
                        sethelptext("Querying...");
                        emailValidation();
                        
                        if(verified && transit && proceed){
                            setTransit(false);
                            invoke("match_vcode", {
                                email: input,
                                attempt : vcode,
                            }).then((message) => {
                                var x = JSON.parse(message);
                                sethelptext(x.response);
                                console.log(x);
                                setProceedF(x.proceed);
                                setTransit(true);
                            })
                        }else{
                            sethelptext("Generate a valid code first!");
                        }
}                       
                    }>

                        <a className='LogRegButt'> Verify </a>

                        
                    </div>

                    <div className='inputBox' style={{marginTop:"-15px"}}>
                        <input type="password" required style={{ marginTop: "-40px" }} onInput={e => setPassword(e.target.value)}></input>
                        <span>New Password</span>
                        <i />
                    </div>

                    <div onClick={(e) => {
                        
                        sethelptext("Querying...");
                        emailValidation();
                        console.log("P2 =>" + proceedF);
                        if(verified && transit && proceedF){
                            setTransit(false);
                            invoke("update_password", {
                                email: input,
                                password : password,
                            }).then((message) => {
                                var x = JSON.parse(message);
                                sethelptext(x.response);
                                console.log(x);
                                setTransit(true);
                            })
                        }else{
                            sethelptext("Generate a valid code first!");
                        }
}                       
                    }>

                        <a className='LogRegButt'> Confirm </a>

                        
                    </div>
                    <a style={{ color: "#FFFFFF", marginTop: "0px" }}>{helptext}</a>
                </form>

            </div>
        </div>
    )

}

export default PassReset_v2;