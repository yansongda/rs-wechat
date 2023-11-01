const logger = wx.getRealtimeLogManager ? wx.getRealtimeLogManager() : null

const info = (...args: any[]) => {
  logger?.info(args)
}

const error = (...args: any[]) => {
  logger?.error(args)
}

export { info, error }