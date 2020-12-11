import React, {Component} from "react";
import ImageControls from "./ImageControls"
import PerlinImage from "./PerlinImage"

import "./ImageGenerator.css"

interface ImageGeneratorProps {}
interface ImageGeneratorState {
  octaves: number;
  persistence: number;
  xrepeat: number;
  yrepeat: number;
  seed: number;
}

class ImageGenerator extends Component<ImageGeneratorProps, ImageGeneratorState> {
  constructor(props: ImageGeneratorProps) {
    super(props);
    this,this.handleOctaveChange = this.handleOctaveChange.bind(this);
    this.handlePersistenceChange = this.handlePersistenceChange.bind(this);
    this.handleXrepeatChange = this.handleXrepeatChange.bind(this);
    this.handleYrepeatChange = this.handleYrepeatChange.bind(this);
    this.handleSeedChange = this.handleSeedChange.bind(this);
    this.state = {
      xrepeat: 0.0,
      yrepeat: 0.0,
      persistence: 0.0,
      octaves: 1,
      seed: 0,
    };
  }

  handlePersistenceChange(v: number) {
    this.setState({persistence: v});
  }

  handleOctaveChange(v: number) {
    this.setState({octaves: v});
  }

  handleXrepeatChange(v: number) {
    this.setState({xrepeat: v});
  }

  handleYrepeatChange(v: number) {
    this.setState({yrepeat: v});
  }

  handleSeedChange(v: number) {
    this.setState({seed: v});
  }

  render() {
    return (
      <div className="ImageGenerator">
        <ImageControls
          onOctaveChange={this.handleOctaveChange}
          onPersistenceChange={this.handlePersistenceChange}
          onXrepeatChange={this.handleXrepeatChange}
          onYrepeatChange={this.handleYrepeatChange}
          onSeedChange={this.handleSeedChange}
          octaveValue={this.state.octaves}
          persistenceValue={this.state.persistence}
          xrepeatValue={this.state.xrepeat}
          yrepeatValue={this.state.yrepeat}
          seedValue={this.state.seed}
        />

        <PerlinImage
          octaves={this.state.octaves}
          persistence={this.state.persistence}
          xrepeat={this.state.xrepeat}
          yrepeat={this.state.yrepeat}
        />
      </div>
    );
  }
}

export default ImageGenerator;