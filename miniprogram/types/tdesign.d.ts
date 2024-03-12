import { Tap } from './wechat'

export interface TabbarTap<T, D> extends Tap<T, D> {
  detail: {
    value: string
  }
}

export interface SwipeTap<DETAIL, T, D> extends Tap<T, D> {
  detail: DETAIL
}

export interface CollapseChange<T, D> extends Tap<T, D> {
  detail: {
    value: number[]
  }
}
