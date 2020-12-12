import React, {ChangeEvent, Component} from "react";

import AppContent from "./AppContent";
import "./AppPage.css";

class App extends Component {
  render() {
    return(
      <div className="AppPage">
        <h1 className="PageTitle">Perlin Images</h1>
        <AppContent/>
      </div>
    );
  }
}

export default App;