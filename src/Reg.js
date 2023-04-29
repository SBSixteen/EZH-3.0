//Params
import { Canvas, Camera } from "@react-three/fiber"
import { Suspense } from "react";
import { OrbitControls, Stars, PerspectiveCamera} from "@react-three/drei";
import * as THREE from "three"
//import "./CSS Bank/LogReg.css"

function Box(){
    return(
        <mesh>
            <boxBufferGeometry attach={"geometry"}/>
            <meshLambertMaterial attach="material" color="hotpink"/>
        </mesh>
    )
}

function randomizer(num =8){
    var numVal = - Math.random()*num+Math.random() *num();
    return numVal;
}

var uspeed = 0.001;
var createCarPos = true;
var setTintNum = true;

//Building
function setTintColor(){
    if(setTintNum){
        setTintNum=false;
        var setColor = 0x000000;
    }else{
        setTintNum= true;
        var setColor = 0x000000;
    }

    return setColor;
}

function init(){

    var segments = 2;

    for(var i = 0; i<100; i++){
        <mesh>
            <boxBufferGeometry 
            args={[1,0,0,segments,segments,segments]} 
            attach={"geometry"}
            />
            
            <meshStandardMaterial color={setTintColor()} 
            wireframe={false} 
            side={THREE.DoubleSide}/>
           
            <meshLambertMaterial 
            color={0x000000}
            wireframe={false}
            transparent={true}
            opacity={0.3}
            side={THREE.DoubleSide}
            />
        </mesh>
    }

}

function Reg() {

    return (

        <div>

        <Suspense fallback={null}>
            <Canvas shadows>
                <PerspectiveCamera 
                position={[0, 2,14]} 
                aspect={window.innerWidth/window.innerHeight}
                fov={20}
                near={1}
                far={500}
                />

                <ambientLight intensity={0.5}></ambientLight>

            {/* Scene */}
            <mesh>
                <fog color={"red"} near={10} far={16}></fog>
            </mesh>
            {/* City */}
            <mesh> {init()}</mesh>
            {/* Smoke */}
            <mesh></mesh>
            {/* town */}
            <mesh></mesh>
            </Canvas>
        </Suspense>

        </div>


    )

}

export default Reg;