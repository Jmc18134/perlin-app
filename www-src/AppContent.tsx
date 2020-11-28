import React from "react";

import IntroParagraph from "./IntroParagraph";
import ImageGenerator from "./ImageGenerator";

import "./AppContent.css"

function AppContent(props: {}) {
    return (
      <div className="AppContent">
        <IntroParagraph/>
        <ImageGenerator/>
      </div>
    );
}

export default AppContent;