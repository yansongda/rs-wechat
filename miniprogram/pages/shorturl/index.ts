import api from '@api/shorturl'

Page({
  data: {
    toptipError: '',
    link: '',
    shortlink: ''
  },
  async submit(e: any) {
    await wx.showLoading({title: '生成中', mask: true})

    const { link } = e.detail.value

    api.create(link).then(({shortlink}) => {
      this.setData({link, shortlink})
    }).catch((e) => {
      this.setData({toptipError: e.message});
    }).finally(async () => {
      await wx.hideLoading();
    })
  },
  async copy() {
    if (this.data.shortlink == '') {
      return;
    }

    await wx.setClipboardData({data: this.data.shortlink})
  },
})
