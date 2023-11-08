const l = wx.getRealtimeLogManager ? wx.getRealtimeLogManager() : null

const logger = {
  info: (...args: any[]) => {
    l?.info(args)
  },
  warning: (...args: any[]) => {
    l?.warn(args)
  },
  error: (...args: any[]) => {
    l?.error(args)
  }
}

export default logger
