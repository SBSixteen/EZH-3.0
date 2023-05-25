import React, { useState } from 'react'
import './v2.css'
import { invoke } from "@tauri-apps/api/tauri";
import {useNavigate} from 'react-router-dom';

function Twofa_v2() {

    const navigate = useNavigate();

    const toListing = () => {
      navigate('/listing');
    };

    const [transit, setTransit] = useState(false);
    const [input, setInput] = useState("");
    const [helptext, sethelptext] = useState(false);

    return (
        
        <div>
           <center> <div className='box' style={{ height: "270px" }}>
                <form id="login">
                    <div className='inputBox'>
                        <h2 style={{ fontSize: "22px", marginTop: "-35px" }}>Two Factor Authentication</h2>
                        <input type="text" required style={{ marginTop: "-10px" }} id='2FACODE' onInput={e => setInput(e.target.value)}></input>
                        <span>2FA Code</span>
                        <i />
                    </div>

                    <div onClick={(e) =>{

                        setTransit(false);
                        sethelptext("Querying...");
                        invoke("match_2fa", {
                            email: sessionStorage.getItem("login"),
                            attempt: input,
                          }).then((message) => {
                            var x = JSON.parse(message);
                            sethelptext(x.response);
                            console.log(x);

                            if(x.attempts == 0){

                            }

                            if(x.proceed){
                                toListing();
                            }

                          })


                    }}>

                        <a className='LogRegButt'> Verify </a>

                    </div>
                    <a style={{color: "#FFFFFF", marginTop:"-25px"}}>{helptext}</a>
                </form>

            </div>
            </center>
        </div>
    )

}

export default Twofa_v2;