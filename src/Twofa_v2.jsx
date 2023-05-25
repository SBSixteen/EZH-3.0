import React, { useState } from 'react'
import './v2.css'
import { invoke } from "@tauri-apps/api/tauri";

function Twofa_v2() {

    const [transit, setTransit] = useState(false);
    const [input, setInput] = useState("");
    const [helptext, sethelptext] = useState(false);

    return (
        
        <div>
            <div className='box' style={{ height: "270px" }}>
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
                            email: sessionStorage.getItem("acc"),
                            attempt: input,
                          }).then((message) => {
                            var x = JSON.parse(message);
                            sethelptext(x.response);
                            console.log(x);
                          })


                    }}>

                        <a className='LogRegButt'> Verify </a>

                    </div>
                    <a style={{color: "#FFFFFF", marginTop:"-25px"}}>{helptext}</a>
                </form>

            </div>
        </div>
    )

}

export default Twofa_v2;