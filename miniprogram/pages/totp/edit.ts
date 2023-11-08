import api from '@api/totp'

Page({
  data: {
    toptipError: '',
    dialogShow: false,
    dialogButtons: [{ text: '取消' }, { text: '重试' }],
    id: 0,
    issuer: '',
    username: ''
  },
  onLoad(query: any) {
    this.data.id = Number(query.id || 0)
  },
  async onShow() {
    await wx.showLoading({ title: '加载中' })

    api
      .detail(this.data.id)
      .then(({ id, issuer, username }) => {
        this.setData({ id, issuer: issuer ?? '', username: username ?? '' })
      })
      .catch(() => {
        this.setData({ dialogShow: true })
      })
      .finally(() => wx.hideLoading())
  },
  async submit(e: any) {
    await wx.showToast({ title: '更新中', icon: 'loading', mask: true, duration: 3000 })

    api
      .update({ id: this.data.id, ...e.detail.value } as ITotpUpdateRequest)
      .then(() => {
        wx.showToast({
          title: '修改成功',
          icon: 'success',
          mask: true,
          success: () => {
            setTimeout(() => wx.navigateBack(), 1500)
          }
        })
      })
      .catch(() => {
        this.setData({ toptipError: e.message })
      })
  },
  async cancel() {
    await wx.navigateBack()
  },
  async dialogTap(e: any) {
    this.setData({ dialogShow: false })

    const { index } = e.detail

    if (index === 1) {
      await this.onShow()
    }
  },
  dialogClose() {
    this.setData({ dialogShow: false })
  }
})
