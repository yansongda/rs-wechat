import type { Tap } from 'miniprogram/types/wechat'

interface CurrentTargetDataset {
  url: string
}

Page({
  data: {
    tools: [
      {
        name: 'TOTP身份验证器',
        icon: 'lock',
        url: '/pages/totp/index'
      },
      {
        name: '短链生成服务',
        icon: 'link',
        url: '/pages/shorturl/index'
      },
      {
        name: '时间工具',
        icon: 'time',
        url: '/pages/time/index'
      }
    ]
  },
  async navigate(e: Tap<CurrentTargetDataset, unknown>) {
    const { url } = e.currentTarget.dataset

    await wx.navigateTo({ url })
  }
})
