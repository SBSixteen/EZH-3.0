import { useState } from "react";
import Login from "./Login.jsx";
import PricingPlan from "./PricingPlan.jsx";
import {
  createStyles,
  Navbar,
  Group,
  Code,
  getStylesRef,
  rem,
} from "@mantine/core";
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
} from "@tabler/icons-react";
import { MantineLogo } from "@mantine/ds";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Sidebar from "./Sidebar.jsx";
import "./Home.css";

/*const useStyles = createStyles((theme) => ({
  header: {
    paddingBottom: theme.spacing.md,
    marginBottom: `calc(${theme.spacing.md} * 1.5)`,
    borderBottom: `${rem(1)} solid ${
      theme.colorScheme === "light" 
    }`,
  },

  footer: {
    paddingTop: theme.spacing.md,
    marginTop: theme.spacing.md,
    borderTop: `${rem(1)} solid ${
      theme.colorScheme === "light" 
    }`,
  },

  link: {
    ...theme.fn.focusStyles(),
    display: "flex",
    alignItems: "center",
    textDecoration: "none",
    fontSize: theme.fontSizes.sm,
    color:
      theme.colorScheme === "light",
    padding: `${theme.spacing.xs} ${theme.spacing.sm}`,
    borderRadius: theme.radius.sm,
    fontWeight: 500,

    "&:hover": {
      backgroundColor:
        theme.colorScheme === "light",
      color: theme.colorScheme === "light",

      [`& .${getStylesRef("icon")}`]: {
        color: theme.colorScheme === "light",
      },
    },
  },

  linkIcon: {
    ref: getStylesRef("icon"),
    color:
      theme.colorScheme === "light",
    marginRight: theme.spacing.sm,
  },

  linkActive: {
    "&, &:hover": {
      backgroundColor: theme.fn.variant({
        variant: "light",
        color: theme.primaryColor,
      }).background,
      color: theme.fn.variant({ variant: "light", color: theme.primaryColor })
        .color,
      [`& .${getStylesRef("icon")}`]: {
        color: theme.fn.variant({ variant: "light", color: theme.primaryColor })
          .color,
      },
    },
  },
}));
 
const data = [
  { link: "", label: "Home", icon: IconHome },
  { link: "", label: "Notifications", icon: IconBellRinging },
  { link: "", label: "Profile", icon: IconUserCircle },
  { link: "", label: "Subscription", icon: IconReceipt2 },
  { link: "", label: "Dataset", icon: IconDatabaseExport },
  { link: "", label: "Settings", icon: IconSettings },
];*/

function Home() {
    
  /*const { classes, cx } = useStyles();
  const [active, setActive] = useState("Billing");
  const links = data.map((item) => (
    <a
      className={cx(classes.link, {
        [classes.linkActive]: item.label === active,
      })}
      href={item.link}
      key={item.label}
      onClick={(event) => {
        event.preventDefault();
        setActive(item.label);
      }}
    >
        
      <item.icon className={classes.linkIcon} stroke={1.5} />
      <span>{item.label}</span>
    </a>
  ));*/

  return (

    <div className="container-new">
      <Sidebar/>
      <div className="others">
        other pages
      </div>
    </div>
   /*<div>
    <div style={{width:"20%",float:"left"}}>
    <Navbar height={700} width={{ sm: 300 }} p="md">
      <Navbar.Section grow>
        <Group className={classes.header} position="apart">
          <img
            style={{ paddingLeft: "15px" }}
            margin-left="auto"
            width="150px"
            height="75px"
            src="Logo_Ezhire.svg"
            className="logo react"
            alt="Tauri logo"
          />
        </Group>
        {links}
      </Navbar.Section>

      <Navbar.Section className={classes.footer}>
        <a
          href="#"
          className={classes.link}
          onClick={(event) => event.preventDefault()}
        >
          <IconLogout className={classes.linkIcon} stroke={1.5} />
          <span>Logout</span>
        </a>
      </Navbar.Section>
    </Navbar>
    </div>
    <div style={{width:"80%",float:"right"}}>
    </div>
    </div>*/
    
  );
}
export default Home;
