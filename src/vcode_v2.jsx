import React from 'react'
import './v2.css'

function vcode_v2() {

    return (

        <div>

            <div>
                {/* {Someone get the logo loaded!!} */}
                <img src="/EZH Logo.svg" />
            </div>


            <title>EZHIRE</title>
            <div className='box' style={{height : "600px"}}>

                <form>

                    <div className='inputBox'>
                        <h2>Sign In</h2>
                        <input type="text" required></input>
                        <span>Verification Code</span>
                        <i />
                    </div>

                    <input type='submit' value="Register" style={{marginTop:"55px"}}/>
                    <div className='links'>
                        {/* Temporary ? for ease of access */}
                        <a href="/regv2">Back</a> 
                        <a href="/loginv2">Got no code?</a>
                    </div>

                    {/* Add Code Verification*/}

                </form>

            </div>


        </div>

    );

}

export default vcode_v2;