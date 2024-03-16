import { CollapseChange, DateTimePickerConfirm } from "miniprogram/types/tdesign"

interface Query {
  id?: string
}

Page({
  data: {
    dateVisible: false,
    collapseActive: [] as number[],
    data: {
      id: 0,
      date: '2024-03-11',
    },
  },
  onLoad(query: Query) {
    this.data.data.id = Number(query.id || 0)
  },
  onShow() {
    // todo：查询
  },
  dateCellClick() {
    this.setData({dateVisible: true})
  },
  dateConfirm(e: DateTimePickerConfirm) {
    this.setData({['data.date']: e.detail.value})
  },
  dateCancel() {
    this.setData({dateVisible: false})
  },
  collapseChange(e: CollapseChange) {
    this.setData({collapseActive: e.detail.value})
  }
})