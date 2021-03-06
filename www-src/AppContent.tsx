import React from "react";

import ImageGenerator from "./ImageGenerator";

import "./AppContent.css"

function AppContent(props: {}) {
    return (
      <div className="AppContent">
        <p>
          This is an app which uses Perlin noise to generate images.
          The Perlin noise function is written in Rust that has been compiled to webassembly. 
          The options below can adjust the persistance, number of octaves, seed, and other parameters
          used in generating the noise. A great explanation of Perlin noise and these parameters can be
          found <a href="http://adrianb.io/2014/08/09/perlinnoise.html">here</a>.
        </p>

        <ImageGenerator/>

        <p>
          Each <span className="KeyPhrase">Octave</span> adds another layer of perlin noise
          which is averaged into the image.<br></br>
        </p>

        <p>
          <span className="KeyPhrase">Peristence</span> is a measure of how much each successive octave 
          influences the final value for each pixel.<br></br>
        </p>

        <p>
          <span className="KeyPhrase">Seed</span> is a seed for the random number generator which
          shuffles the hash table used to get the pseudorandom "gradient vectors" in the original
          Perlin Noise implementation.<br></br>
        </p>

        <p>
          Perlin noise actually ends up being 0 when all the inputs are integers, so we divide
          the inputs by a <span className="KeyPhrase">Period</span> (also called Frequency).
          This value essentially changes the spacing of the grid which the gradients are placed on.<br></br>
        </p>
        
        <p className="Footer">
          This app was created with React, Rust WebAssembly, and TypeScript.
          You can find GitHub the repo <a href="https://github.com/Jmc18134/perlin-app">here</a>.
        </p>
      </div>
    );
}

export default AppContent;