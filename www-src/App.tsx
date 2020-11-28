import React, {ChangeEvent, Component} from "react";

import Title from "./Title";
import AppContent from "./AppContent";
import "./AppPage.css";

class App extends Component {
  render() {
    return(
      <div className="AppPage">
        <Title/>
        <AppContent/>
      </div>
    );
  }
}

export default App;