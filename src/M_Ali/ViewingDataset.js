import { GridComponent, ColumnsDirective, ColumnDirective, Selection, Search, Inject, Toolbar, Sort, Filter } from '@syncfusion/ej2-react-grids';
import * as React from 'react';
import  Header  from './Components/Header';
import './ViewingDataset.css';
import { useNavigate } from "react-router-dom";

function ViewingDataset() {
    let gridInstance;
    
    const settings = { mode: 'Row', column: 'checkbox'}; 
    const toolbarOptions = [{ text: 'Compare Selected', tooltipText: 'Compare Selected', prefixIcon: 'e-compare', id: 'compareselected' },'Search'];

    const clickHandler = (args) => {
        if (args.item.id === 'compareselected') {
            gotoRegister();
        }
        }
    let navigate = useNavigate();
    const gotoRegister = () =>{

        let path = "/Register";
        navigate(path);
    }

    let data = [
        {'Name': 'Nirav Joshi','EmailID': 'nirav@gmail.com', 'Contact': '10011'},
        {'Name': 'Sunil Joshi','EmailID': 'sunil@gmail.com', 'Contact': '10012'},
        {'Name': 'Andrew McDownland','EmailID': 'andrew@gmail.com', 'Contact': '10013' },
        {'Name': 'Christopher Jamil','EmailID': 'jamil@gmail.com', 'Contact': '10014'}
        ];
        
    const titlefrompreviouspage = 'Dataset A';


    
    return( 
        
        <div>
            <div className="newlogo">
      <a href="" target="_blank">
        <img src="/Logo_Ezhire.svg" className="logo tauri" alt="Tauri logo" />
      </a>
    </div>
            <Header title= {titlefrompreviouspage} />
            <div style={{
                padding: '100px',
                paddingTop: '20px',
                boxSizing: 'content-box',
            }}>
                <GridComponent 
                dataSource={data}
                enableHover={false}
                selectionSettings={settings}
                toolbar={toolbarOptions}
                allowSorting
                toolbarClick={clickHandler}
            >
                <ColumnsDirective>
                    <ColumnDirective type= 'checkbox' width= '50' />
                    <ColumnDirective field='Name' headerText='Name'/>
                    <ColumnDirective field='EmailID' headerText='Email ID' />
                    <ColumnDirective field='Contact' headerText='Contact' />
                </ColumnsDirective>
                <Inject services={[Search, Selection, Toolbar, Sort, Filter]} />
            </GridComponent>
            </div>
        </div>
        
    )
};
export default ViewingDataset;
