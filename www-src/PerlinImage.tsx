import React, {PureComponent} from "react";

import "./PerlinImage.css"

let { PerlinGen } = await import("../perlin/perlin");
let { memory } = await import("../perlin/perlin_bg.wasm");

interface PerlinImageProps {
    octaves: number;
    persistence: number;
}

class PerlinImage extends PureComponent<PerlinImageProps, {}> {
    canvasRef: React.RefObject<HTMLCanvasElement>
    backingPixels: any

    constructor(props: PerlinImageProps) {
      super(props);
      this.canvasRef = React.createRef();
      this.backingPixels = PerlinGen.new(256, 256, 42);
    }

    componentDidUpdate() {
      const { octaves, persistence } = this.props;
      this.backingPixels.fill_self(
        octaves,
        persistence,
      );

      const pixelPtr = this.backingPixels.get_pixels();
      const pixels = new Uint8ClampedArray(memory.buffer, pixelPtr, 4 * 256 * 256);
 
      const newData = new ImageData(pixels, 256, 256);

      const canvas = this.canvasRef.current;
      const ctx = canvas.getContext('2d');

      ctx.putImageData(newData, 0, 0);
    }
  
    render() {
      return (
        <canvas className="PerlinImage" width="256" height="256" ref={this.canvasRef}></canvas>
      );
    }
}

export default PerlinImage;