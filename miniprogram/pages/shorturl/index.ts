import api from '@api/shorturl'

Page({
  data: {
    link: '',
    shortlink: ''
  },
  async submit(e: any) {
    const { link } = e.detail.value

    const { shortlink } = await api.create(link)

    if ((shortlink ?? '') == '') {
      wx.showToast({title: '生成失败', duration: 2000, icon: 'error'})
      return;
    }

    this.setData({
      link,
      shortlink,
    })
  },
  copy() {
    if (this.data.shortlink == '') {
      return;
    }

    wx.setClipboardData({data: this.data.shortlink})
  },
})
