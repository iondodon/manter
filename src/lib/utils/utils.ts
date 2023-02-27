export const arrayBufferToString = (buf: ArrayBuffer) => {
  return String.fromCharCode.apply(null, new Uint8Array(buf));
};

export const webglIsSupported = () => {
  // looks like xterm-addon-webgl is not working with webgl2
  var canvas = document.createElement("canvas");
  var gl = canvas.getContext("webgl2");
  if (gl && gl instanceof WebGL2RenderingContext) {
    return true;
  } else {
    return false;
  }
};
