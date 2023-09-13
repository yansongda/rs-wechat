import api from '@api/shorturl'

Page({
  data: {
    link: '',
    shortlink: ''
  },
  async submit(e: any) {
    await wx.showToast({title: '生成中', icon: 'loading', mask: true, duration: 3000})

    const { link } = e.detail.value

    const { shortlink } = await api.create(link)

    if ((shortlink ?? '') == '') {
      await wx.showToast({title: '生成失败', icon: 'error'})
      return;
    }

    this.setData({
      link,
      shortlink,
    })

    await wx.hideToast()
  },
  async copy() {
    if (this.data.shortlink == '') {
      return;
    }

    await wx.setClipboardData({data: this.data.shortlink})
  },
})
