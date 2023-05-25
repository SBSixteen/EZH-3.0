import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { makeStyles } from "@material-ui/core/styles";
import {
  TextField,
  Typography,
  Divider,
  FormControlLabel,
  Switch,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
} from "@material-ui/core";
import AccountCircleIcon from "@material-ui/icons/AccountCircle";
import EmailIcon from "@material-ui/icons/Email";
import LockIcon from "@material-ui/icons/Lock";
import NotificationsIcon from "@material-ui/icons/Notifications";
import Button from "./Components/Button";
import Sidebar from "./Sidebar.jsx";
import "./Home.css";
import "./style.css";
import { Key } from "@mui/icons-material";
import { IconKey, IconKeyboard } from "@tabler/icons-react";

const useStyles = makeStyles((theme) => ({
  root: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    paddingTop: theme.spacing(4),
    
  },
  form: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    background: "#cd1985b5",
    borderRadius: "30px"
  },
  section: {
    marginTop: theme.spacing(4),
    marginBottom: theme.spacing(4),
    width: "100%",
    maxWidth: 600,
  },
  field: {
    margin: theme.spacing(2),
    minWidth: 200,
  },
  button: {
    marginTop: theme.spacing(4),
  },
}));

const Settings = () => {
  const classes = useStyles();
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [notifications, setNotifications] = useState(false);
  const [twofactor, setTwoFactor] = useState(false);
  const [link, setLink] = useState("/passreset_v2");
  const navigate = useNavigate();
  const handleClick = () => navigate(link);

  const handleSubmit = (event) => {
    event.preventDefault();
    // Send data to backend for processing
    console.log({ name, email, password, notifications });
  };

  return (
    <>
      <div className="container-new">
        <Sidebar />
        <div className="others">
          <div className={classes.root}>
            <h1 style = {{color:"white"}} >Settings</h1>
            <Divider className={classes.section} />
            <form className={classes.form} onSubmit={handleSubmit}>
              <h1>Account Information</h1>
              <List className={classes.section}>
                <ListItem>
                  <ListItemIcon>
                    <AccountCircleIcon />
                  </ListItemIcon>
                  <ListItemText
                    primary="Name"
                    secondary="Your name as it appears on your account"
                  />
                  <TextField
                    variant="outlined"
                    className={classes.field}
                    value={name}
                    onChange={(event) => setName(event.target.value)}
                  />
                </ListItem>
                <ListItem>
                  <ListItemIcon>
                    <EmailIcon />
                  </ListItemIcon>
                  <ListItemText
                    primary="Email"
                    secondary="The email address associated with your account"
                  />
                  <TextField
                    variant="outlined"
                    className={classes.field}
                    value={email}
                    onChange={(event) => setEmail(event.target.value)}
                  />
                </ListItem>
                <ListItem>
                  <ListItemIcon>
                    <LockIcon />
                  </ListItemIcon>
                  <ListItemText
                    primary="Password"
                    secondary="Change your password"
                  />
                  <div>
                    <a className="LogRegButt" href="passreset_v2">
                      {" "}
                      Change Password{" "}
                    </a>
                  </div>
                </ListItem>
              </List>
              <h1>Notification Settings</h1>
              <List className={classes.section}>
                <ListItem>
                  <ListItemIcon>
                    <NotificationsIcon />
                  </ListItemIcon>
                  <ListItemText
                    primary="Enable Notifications"
                    secondary="Receive notifications for important updates"
                  />
                  <FormControlLabel
                    className={classes.formControlLabel}
                    control={
                      <Switch
                        color="primary"
                        checked={notifications}
                        onChange={() => setNotifications(!notifications)}
                      />
                    }
                    label=""
                  />
                </ListItem>
                <ListItem>
                  <ListItemIcon>
                    <Key />
                  </ListItemIcon>
                  <ListItemText
                    primary="Two Factor Authentication"
                    secondary="Enable or Disable double layered protection"
                  />
                  <FormControlLabel
                    className={classes.formControlLabel}
                    control={
                      <Switch
                        color="primary"
                        checked={twofactor}
                        onChange={() => setTwoFactor(!twofactor)}
                      />
                    }
                    label=""
                  />
                </ListItem>
              </List>
              <div>
                <a className="LogRegButt" href="]">
                  {" "}
                  Save Changes{" "}
                </a>
              </div>
            </form>
          </div>
        </div>
      </div>
    </>
  );
};

export default Settings;
