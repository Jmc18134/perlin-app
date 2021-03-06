import React, {ChangeEvent, Component, KeyboardEvent, InputHTMLAttributes} from "react";

import "./ImageControls.css";

interface ImageControlProps {
    onOctaveChange(
      newOctaves: number
    ): void;
    onPersistenceChange(
      newPersistence: number
    ): void;
    onSeedChange(
      newSeed: number
    ): void;
    onPeriodChange(
      newPeriod: number
    ): void;

    seedValue: number
    octaveValue: number;
    persistenceValue: number;
    periodValue: number;
}

class ImageControls extends Component<ImageControlProps, {}> {
    constructor(props: ImageControlProps) {
      super(props);
      this.onOctaveChange = this.onOctaveChange.bind(this);
      this.onPersistenceChange = this.onPersistenceChange.bind(this);
      this.onSeedChange = this.onSeedChange.bind(this);
      this.onPeriodChange = this.onPeriodChange.bind(this);
      this.isFloatKey = this.isFloatKey.bind(this);
      this.isIntKey = this.isIntKey.bind(this);
    }
  
    onOctaveChange(e: ChangeEvent<HTMLInputElement>) {
      let newOctaves: number = parseInt(e.target.value);
      this.props.onOctaveChange(newOctaves);
    }
  
    onPersistenceChange(e: ChangeEvent<HTMLInputElement>) {
      let newPers: number = parseFloat(e.target.value);
      this.props.onPersistenceChange(newPers);
    }

    onSeedChange(e: ChangeEvent<HTMLInputElement>) {
      let newRepeat: number = parseInt(e.target.value);
      this.props.onSeedChange(newRepeat);
    }

    onPeriodChange(e: ChangeEvent<HTMLInputElement>) {
      let newRepeat: number = parseFloat(e.target.value);
      this.props.onPeriodChange(newRepeat);
    }

    isIntKey(e: KeyboardEvent) {
      let key = e.key;
      if (isNaN(key as any)) {
        e.preventDefault();
      }
    }

    isFloatKey(e: KeyboardEvent) {
      let key = e.key;
      let input = e.target as HTMLInputElement;
      if (isNaN(key as any) && !(key == '.' && !input.value.includes("."))) {
          e.preventDefault();
      }
    }
  
    render() {
      return (
        <div className="ImageControls">
          <div className="LabelInputPair">
            <label htmlFor="octaves">No. of octaves:</label>
            <input
              type="number" name="octaves" min={0} step={1}
              onChange={this.onOctaveChange}
              onKeyPress={this.isIntKey}
              value={this.props.octaveValue}
            />
          </div>

          <div className="LabelInputPair">
            <label htmlFor="persistence">Persistence:</label>
            <input 
              type="number" name="persistence" min={0.0} step={0.1}
              onChange={this.onPersistenceChange}
              onKeyPress={this.isFloatKey}
              value={this.props.persistenceValue}
            />
          </div>

          <div className="LabelInputPair">
            <label htmlFor="seed">Seed:</label>
            <input 
              type="number" name="seed" min={0} step={1}
              onChange={this.onSeedChange}
              onKeyPress={this.isIntKey}
              value={this.props.seedValue}
            />
          </div>

          <div className="LabelInputPair">
            <label htmlFor="period">Period:</label>
            <input 
              type="number" name="period" min={1} step={1}
              onChange={this.onPeriodChange}
              onKeyPress={this.isFloatKey}
              value={this.props.periodValue}
            />
          </div>
        </div>
      );
    }
}

export default ImageControls;