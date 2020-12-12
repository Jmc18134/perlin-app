import React, {PureComponent} from "react";
import { perlingen_fill_self } from "../perlin/perlin_bg.wasm";

import "./PerlinImage.css"

let { PerlinGen } = await import("../perlin/perlin");
let { memory } = await import("../perlin/perlin_bg.wasm");

interface PerlinImageProps {
    octaves: number;
    persistence: number;
    seed: number;
    period: number;
}

class PerlinImage extends PureComponent<PerlinImageProps, {}> {
    canvasRef: React.RefObject<HTMLCanvasElement>
    backingPixels: any

    constructor(props: PerlinImageProps) {
      super(props);
      this.canvasRef = React.createRef();
      this.backingPixels = PerlinGen.new(256, 256, this.props.seed);
    }

    componentDidUpdate(prevProps: PerlinImageProps, _prevState: {}) {
      const { octaves, persistence, seed, period } = this.props;

      if (seed !== prevProps.seed) {
        this.backingPixels.change_seed(seed);
      }

      this.backingPixels.fill_self(
        octaves,
        persistence,
        period
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