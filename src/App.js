import { useState } from 'react';
import Login from './Login.jsx';
import Home from './Home.js';
import Register from './Register';
import Settings from './Settings';
import PaymentForm from './PaymentForm.jsx';
import { createStyles, Navbar, Group, Code, getStylesRef, rem } from '@mantine/core';
import {
  IconBellRinging,
  IconFingerprint,
  IconKey,
  IconSettings,
  Icon2fa,
  IconDatabaseImport,
  IconReceipt2,
  IconSwitchHorizontal,
  IconLogout,
  IconHome,
  IconSubscript,
  IconDatabaseExport,
  IconPhoto,
  IconUserCircle,
} from '@tabler/icons-react';
import { MantineLogo } from '@mantine/ds';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import PassReset from './PassReset.jsx';

const useStyles = createStyles((theme) => ({
  header: {
    paddingBottom: theme.spacing.md,
    marginBottom: `calc(${theme.spacing.md} * 1.5)`,
    borderBottom: `${rem(1)} solid ${
      theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
    }`,
  },

  footer: {
    paddingTop: theme.spacing.md,
    marginTop: theme.spacing.md,
    borderTop: `${rem(1)} solid ${
      theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
    }`,
  },

  link: {
    ...theme.fn.focusStyles(),
    display: 'flex',
    alignItems: 'center',
    textDecoration: 'none',
    fontSize: theme.fontSizes.sm,
    color: theme.colorScheme === 'dark' ? theme.colors.dark[1] : theme.colors.gray[7],
    padding: `${theme.spacing.xs} ${theme.spacing.sm}`,
    borderRadius: theme.radius.sm,
    fontWeight: 500,

    '&:hover': {
      backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
      color: theme.colorScheme === 'dark' ? theme.white : theme.black,

      [`& .${getStylesRef('icon')}`]: {
        color: theme.colorScheme === 'dark' ? theme.white : theme.black,
      },
    },
  },

  linkIcon: {
    ref: getStylesRef('icon'),
    color: theme.colorScheme === 'dark' ? theme.colors.dark[2] : theme.colors.gray[6],
    marginRight: theme.spacing.sm,
  },

  linkActive: {
    '&, &:hover': {
      backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
      color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
      [`& .${getStylesRef('icon')}`]: {
        color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
      },
    },
  },
}));

const data = [
  { link: '', label: 'Home', icon: IconHome },
  { link: '', label: 'Notifications', icon: IconBellRinging },
  { link: '', label: 'Profile', icon: IconUserCircle  },
  { link: '', label: 'Subscription', icon: IconReceipt2},
  { link: '', label: 'Dataset', icon: IconDatabaseExport },
  { link: '', label: 'Settings', icon: IconSettings },
];

function App() {
  
    return (
      <div className={'App'}>
    <BrowserRouter>
<Routes>
<Route path="" element={<Register />} />
<Route path="/Register" element={<Register />} />
<Route path="/login" element={<Login />}/>
<Route path="/Home" element={<Home />} />
<Route path="/PassReset" element={<PassReset />} />
<Route path="/PaymentForm" element={<PaymentForm />} />
<Route path="/Settings" element={<Settings/>} />
navigate(path);
   </Routes>
</BrowserRouter>
      </div>
    );
  }
export default App;