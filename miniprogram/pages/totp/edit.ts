import api from '@api/totp'

Page({
  data: {
    id: 0,
    issuer: '',
    username: '',
  },
  onLoad(query: any) {
    this.data.id = Number(query.id || 0)
  },
  async onShow() {
    const {id, issuer, username} = await api.detail(this.data.id)

    this.setData({
      id,
      issuer: issuer ?? '',
      username: username ?? '',
    })
  },
  async submit(e: any) {
    await wx.showToast({title: '更新中', icon: 'loading', mask: true, duration: 3000})

    await api.update({id: this.data.id, ...e.detail.value} as ITotpUpdateRequest)

    wx.showToast({
      title: '修改成功',
      icon: 'success',
      success: () => {
        setTimeout(() => {
          wx.navigateBack()          
        }, 1500);
      }
    })
  },
  async cancel() {
    await wx.navigateBack()
  }
})