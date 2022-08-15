export const arrayBufferToString = (buf: ArrayBuffer) => {
  return String.fromCharCode.apply(null, new Uint8Array(buf))
}