import React, {PureComponent} from "react";

import "./PerlinImage.css"

import { PerlinGen } from "../perlin";
import { memory } from "../perlin/perlin_bg.wasm"

interface PerlinImageProps {
    octaves: number;
    persistence: number;
    xrepeat: number;
    yrepeat: number;
}

class PerlinImage extends PureComponent<PerlinImageProps, {}> {
    canvasRef: React.RefObject<HTMLCanvasElement>
    backingPixels: PerlinGen

    constructor(props: PerlinImageProps) {
      super(props);
      this.canvasRef = React.createRef();
      this.backingPixels = PerlinGen.new(600, 600);
    }

    componentDidUpdate() {
      const { octaves, persistence, xrepeat, yrepeat } = this.props;
      this.backingPixels.fill_self(
        octaves,
        persistence,
        xrepeat,
        yrepeat
      );

      const pixelPtr = this.backingPixels.get_pixels();
      const pixels = new Uint8ClampedArray(memory.buffer, pixelPtr, 600 * 600);
      const newData = new ImageData(pixels, 600, 600);

      const canvas = this.canvasRef.current;
      const ctx = canvas.getContext('2d');

      ctx.putImageData(newData, 0, 0);
    }
  
    render() {
      return (
        <canvas className="PerlinImage" width="600" height="600" ref={this.canvasRef}></canvas>
      );
    }
}

export default PerlinImage;