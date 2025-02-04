import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

const App = () => {
    return (<p>My App</p>);
}

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);