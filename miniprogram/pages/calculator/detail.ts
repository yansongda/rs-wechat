import { CollapseChange } from "miniprogram/types/tdesign"

interface Query {
  id?: string
}

Page({
  data: {
    date: '2024-03-11',
    collapseActive: [] as number[],
    id: 0,
  },
  onLoad(query: Query) {
    this.data.id = Number(query.id || 0)
  },
  onShow() {

  },
  collapseChange(e: CollapseChange<unknown, unknown>) {
    this.setData({collapseActive: e.detail.value})
  }
})