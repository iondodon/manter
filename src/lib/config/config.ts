export const IS_WINDOWS = navigator.userAgent.includes("Windows")
export const IS_MACINTOSH = navigator.userAgent.includes("Macintosh")
export const IS_LINUX = navigator.userAgent.includes("Linux")
export const IS_UNIX = IS_LINUX || IS_MACINTOSH

export const PTY_WS_ADDRESS = "ws://127.0.0.1:7703"