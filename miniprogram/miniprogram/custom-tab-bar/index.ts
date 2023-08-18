Component({
	data: {
		active: 0,
		list: [
			{
				icon: 'home-o',
				text: '首页',
				url: '/pages/home/home'
			},
			{
				icon: 'user-o',
				text: '我',
				url: '/pages/me/me'
			}
		]
	},

	methods: {
		onChange(event: any) {
			this.setData({ active: event.detail });
			wx.switchTab({
				url: this.data.list[event.detail].url
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
