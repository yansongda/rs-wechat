import { Tap } from "miniprogram/types/wechat"

interface Dataset {
  id: string
}

Page({
  data: {
    items: []
  },
  onShow() {
    // todo: 获取最新10条的数据
  },
  async detail(e: Tap<Dataset, Dataset>) {
    const id = e.currentTarget.dataset.id

    await wx.navigateTo({ url: '/pages/calculator/detail?id=' + id })
  },
  async create() {
    await wx.navigateTo({ url: '/pages/calculator/create' })
  }
})

export default {}
