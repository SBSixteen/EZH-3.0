import React, { useState } from 'react';
import { makeStyles } from '@material-ui/core/styles';
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
} from '@material-ui/core';
import AccountCircleIcon from '@material-ui/icons/AccountCircle';
import EmailIcon from '@material-ui/icons/Email';
import LockIcon from '@material-ui/icons/Lock';
import NotificationsIcon from '@material-ui/icons/Notifications';
import Button from './Components/Button';

const useStyles = makeStyles((theme) => ({
  root: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    paddingTop: theme.spacing(4),
  },
  form: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
  },
  section: {
    marginTop: theme.spacing(4),
    marginBottom: theme.spacing(4),
    width: '100%',
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
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [notifications, setNotifications] = useState(false);
  

  const handleSubmit = (event) => {
    event.preventDefault();
    // Send data to backend for processing
    console.log({ name, email, password, notifications });
  };
  

  return (
    <div className={classes.root}>
      <h1>Settings</h1>
      <Divider className={classes.section} />
      <form className={classes.form} onSubmit={handleSubmit}>
        <h2>Account Information</h2>
        <List className={classes.section}>
          <ListItem>
            <ListItemIcon>
              <AccountCircleIcon />
            </ListItemIcon>
            <ListItemText primary="Name" secondary="Your name as it appears on your account" />
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
            <ListItemText primary="Email" secondary="The email address associated with your account" />
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
            <ListItemText primary="Password" secondary="Change your password" />
            <TextField
              variant="outlined"
              type="password"
              className={classes.field}
              value={password}
              onChange={(event) => setPassword(event.target.value)}
            />
          </ListItem>
        </List>
        <h2>Notification Settings</h2>
        <List className={classes.section}>
          <ListItem>
            <ListItemIcon>
              <NotificationsIcon />
            </ListItemIcon>
            <ListItemText primary="Enable Notifications" secondary="Receive notifications for important updates" />
            <FormControlLabel
            className={classes.formControlLabel}
              control={<Switch color="primary" checked={notifications} onChange={() => setNotifications(!notifications)} />}
              label=""
            />
          </ListItem>
        </List>
        <Button title = "Save Changes">
          </Button>
          </form>
          </div>
          );
          };
          
          export default Settings;
