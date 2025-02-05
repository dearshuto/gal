import * as React from "react";
import Button from "@mui/material/Button";
import Checkbox from "@mui/material/Checkbox";
import FormControlLabel from "@mui/material/FormControlLabel";

const App = () => {
  return (
    <>
      <Button variant="contained">Hello world</Button>
      <FormControlLabel control={<Checkbox defaultChecked />} label="Label" />
    </>
  );
};

export default App;
