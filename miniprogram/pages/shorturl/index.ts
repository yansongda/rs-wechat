import api from '@api/shorturl'

Page({
  data: {
    link: '',
    shortlink: ''
  },
  async submit(e: any) {
    wx.showToast({title: '生成中', icon: 'loading', mask: true, duration: 3000})

    const { link } = e.detail.value

    const { shortlink } = await api.create(link)

    if ((shortlink ?? '') == '') {
      wx.showToast({title: '生成失败', icon: 'error'})
      return;
    }

    this.setData({
      link,
      shortlink,
    })

    wx.hideToast()
  },
  copy() {
    if (this.data.shortlink == '') {
      return;
    }

    wx.setClipboardData({data: this.data.shortlink})
  },
})
