import './listing.css';
 

const data = 
    [
        {
           "email":"gowtham@outlook.com",
           "firstname":"gowtham",
           "lastname":"ss",
           "password":"outlook010"
        },
        {
           "email":"ss@ss.com",
           "firstname":"ss",
           "lastname":"ss",
           "password":"ss"
        },
        {
           "email":"gow@gow.com",
           "firstname":"gow",
           "lastname":"gow",
           "password":"gow"
        }
     ]

 
function Listing() {
    return (
        <div className="Listing">
       <table>
  <thead>
    <tr>
      <th>First name</th>
      <th>Last name</th>
      <th>Password</th>
      <th>Email</th>
    </tr>
  </thead>
  <tbody>
    {data.map(item => {
      return (
        <tr key={item.password}>
          <td>{ item.firstname }</td>
          <td>{ item.lastname }</td>
          <td>{ item.password }</td>
          <td>{ item.email }</td>
          <button>Download</button>
          <button>Delete</button>
        </tr>
      );
    })}
  </tbody>
</table>

        </div>
    );
}
 
export default Listing;