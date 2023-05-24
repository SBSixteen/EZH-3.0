import { useState } from "react";
import Login from "./Login.jsx";
import Home from "./Home.js";
import Register from "./Register";
import Settings from "./Settings";
import PaymentForm from "./PaymentForm.jsx";
import PassReset from "./PassReset.jsx";
import PricingPlan from "./PricingPlan.jsx";
import Dataset from "./Dataset.jsx";
import DynamicTable from "./DynamicTable.jsx";
import Datasetform from "./Datasetform.jsx";
import Nabeel from "./Nabe.jsx";
import Startingpage from "./Startingpage.jsx";
import Comparision from "./Comparision.jsx";
import Login_v2 from "./Login_v2.jsx";
import Registration_v2 from "./Registration_v2.jsx";
import Twofa_v2 from "./Twofa_v2.jsx";
import PassReset_v2 from "./PassReset_v2.jsx";

import {
  createStyles,
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
import { BrowserRouter, Routes, Route } from "react-router-dom";


const useStyles = createStyles((theme) => ({
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
];

function App() {
  return (
    <div className={"App"}>
      <BrowserRouter>
        <Routes>
          <Route path="/register" element={<Register />} />
          <Route path="/login" element={<Login />} />
          <Route path="/home" element={<Home />} />
          <Route path="/pass-reset" element={<PassReset />} />
          <Route path="/payment-form" element={<PaymentForm />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="/subscription" element={<PricingPlan />} />
          <Route path="/dataset" element={<Dataset/>} />
          <Route path="" element={<Nabeel/>} />
          <Route path="/Dataset" element={<Dataset />} />
          <Route path="/DynamicTable" element={<DynamicTable />} />
          <Route path="/Datasetform" element={<Datasetform />} />
          <Route path="/logv2" element={<Login_v2 />} />
          <Route path="/regv2" element={<Registration_v2 />} />
          <Route path="/2fa" element={<Twofa_v2 />} />
          <Route path="/passreset_v2" element={<PassReset_v2 />} />
          {/* <Route path="/Comparision" element={<Comparision />} /> */}
          navigate(path);
        </Routes>
      </BrowserRouter>
    </div>
  );
}
export default App;
