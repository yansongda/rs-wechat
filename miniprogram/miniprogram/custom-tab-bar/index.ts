Component({
	data: {
		active: 0,
		list: [
			{
				text: '首页',
        iconPath: '/images/icon/app.png',
        selectedIconPath: '/images/icon/app-selected.png',
				url: '/pages/home/home'
			},
			{
				text: '我',
        iconPath: '/images/icon/user.png',
        selectedIconPath: '/images/icon/user-selected.png',
				url: '/pages/me/me'
			}
		]
	},

	methods: {
		onChange(event: any) {
      this.setData({ active: event.detail.index });
			wx.switchTab({
				url: this.data.list[event.detail.index].url
			});
		},

		init() {
      const page = getCurrentPages().pop();
			this.setData({
        // @ts-ignore
				active: this.data.list.findIndex(item => item.url === `/${page.route}`)
			});
		}
	}
});
