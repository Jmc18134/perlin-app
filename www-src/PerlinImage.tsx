import React, {Component} from "react";

import "./PerlinImage.css"

interface PerlinImageProps {
    persistence: number;
    octaves: number;
    xrepeat: number;
    yrepeat: number;
}

class PerlinImage extends Component<PerlinImageProps, {}> {
    constructor(props: PerlinImageProps) {
      super(props);
    }
  
    render() {
      return (
        <canvas className="PerlinImage" width="600" height="600"></canvas>
      );
    }
}

export default PerlinImage;