import React, {Component} from "react";

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
        <canvas></canvas>
      );
    }
}

export default PerlinImage;