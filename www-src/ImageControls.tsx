import React, {ChangeEvent, Component} from "react";

import "./ImageControls.css";

interface ImageControlProps {
    onOctaveChange(
      newOctaves: number
    ): void;
    onPersistenceChange(
      newPersistence: number
    ): void;
    onXrepeatChange(
      newRepeat: number
    ): void;
    onYrepeatChange(
      newRepeat: number
    ): void;
}

class ImageControls extends Component<ImageControlProps, {}> {
    constructor(props: ImageControlProps) {
      super(props);
      this.onOctaveChange = this.onOctaveChange.bind(this);
      this.onPersistenceChange = this.onPersistenceChange.bind(this);
      this.onXrepeatChange = this.onXrepeatChange.bind(this);
      this.onYrepeatChange = this.onYrepeatChange.bind(this);
    }
  
    onOctaveChange(e: ChangeEvent<HTMLInputElement>) {
      let newOctaves: number = parseInt(e.target.value);
      this.props.onOctaveChange(newOctaves);
    }
  
    onPersistenceChange(e: ChangeEvent<HTMLInputElement>) {
      let newPers: number = parseFloat(e.target.value);
      this.props.onPersistenceChange(newPers);
    }
  
    onYrepeatChange(e: ChangeEvent<HTMLInputElement>) {
      let newRepeat: number = parseFloat(e.target.value);
      this.props.onYrepeatChange(newRepeat);
    }
  
    onXrepeatChange(e: ChangeEvent<HTMLInputElement>) {
      let newRepeat: number = parseFloat(e.target.value);
      this.props.onXrepeatChange(newRepeat);
    }
  
    render() {
      return (
        <div className="ImageControls">
          <div className="LabelInputPair">
            <label htmlFor="xrepeat">X-axis Repeat:</label>
            <input
              type="number" name="xrepeat" min={0} step={0.001}
              onChange={this.onXrepeatChange}
            />
          </div>

          <div className="LabelInputPair">
            <label htmlFor="yrepeat">Y-axis Repeat:</label>
            <input
              type="number" name="yrepeat" min={0} step={0.001}
              onChange={this.onYrepeatChange}
            />
          </div>

          <div className="LabelInputPair">
            <label htmlFor="octaves">No. of octaves:</label>
            <input
              type="number" name="octaves" min={1} step={1}
              onChange={this.onOctaveChange}
            />
          </div>

          <div className="LabelInputPair">
            <label htmlFor="persistence">Persistence:</label>
            <input 
              type="number" name="persistence" min={0.001} step={0.001}
              onChange={this.onPersistenceChange}
            />
          </div>
        </div>
      );
    }
}

export default ImageControls;