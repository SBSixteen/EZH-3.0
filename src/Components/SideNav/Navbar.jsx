import React from 'react'
import * as FAIcons from "react-icons/fa"
import {Link} from 'react-router-dom'

function Navbar(){

    return(
        <>
            <div className='navbar'>
                <Link to="#" className='menu-bars'>
                    <FAIcons.FaBars/>
                </Link>
            </div>
        </>
    )

}

export default Navbar;