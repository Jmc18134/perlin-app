import React from "react";

function IntroParagraph(props: {}) {
    return (
     <p className="IntroParagraph">
       This is an app which makes use of Perlin noise in order
       to generate images. The Perlin noise function is written
       in Rust that has been compiled to webassembly (the image data
       itself is also generated within webassembly, and passed to JS). 
       The options below can adjust the  persistance, repeats, and
       number of octaves used.
     </p>
    );
}

export default IntroParagraph;