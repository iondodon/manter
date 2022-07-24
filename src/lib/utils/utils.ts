export const ab2str = (buf: ArrayBuffer) => {
  return String.fromCharCode.apply(null, new Uint8Array(buf))
}