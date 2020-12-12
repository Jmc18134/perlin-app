import React, {Component} from "react";
import ImageControls from "./ImageControls"
import PerlinImage from "./PerlinImage"

import "./ImageGenerator.css"

interface ImageGeneratorProps {}
interface ImageGeneratorState {
  octaves: number;
  persistence: number;
  seed: number;
  period: number;
}

class ImageGenerator extends Component<ImageGeneratorProps, ImageGeneratorState> {
  constructor(props: ImageGeneratorProps) {
    super(props);
    this,this.handleOctaveChange = this.handleOctaveChange.bind(this);
    this.handlePersistenceChange = this.handlePersistenceChange.bind(this);
    this.handleSeedChange = this.handleSeedChange.bind(this);
    this.handlePeriodChange = this.handlePeriodChange.bind(this);
    this.state = {
      persistence: 0.0,
      octaves: 1,
      seed: 1,
      period: 1
    };
  }

  handlePersistenceChange(v: number) {
    this.setState({persistence: v});
  }

  handleOctaveChange(v: number) {
    this.setState({octaves: v});
  }

  handleSeedChange(v: number) {
    this.setState({seed: v});
  }

  handlePeriodChange(v: number) {
    this.setState({period: v});
  }

  render() {
    return (
      <div className="ImageGenerator">
        <ImageControls
          onOctaveChange={this.handleOctaveChange}
          onPersistenceChange={this.handlePersistenceChange}
          onSeedChange={this.handleSeedChange}
          onPeriodChange={this.handlePeriodChange}
          octaveValue={this.state.octaves}
          persistenceValue={this.state.persistence}
          seedValue={this.state.seed}
          periodValue={this.state.period}
        />

        <PerlinImage
          octaves={this.state.octaves}
          persistence={this.state.persistence}
          seed={this.state.seed}
          period={this.state.period}
        />
      </div>
    );
  }
}

export default ImageGenerator;