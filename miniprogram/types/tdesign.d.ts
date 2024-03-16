import { Tap } from './wechat'

export interface TabbarTap<T, D> extends Tap<T, D> {
  detail: {
    value: string
  }
}

export interface SwipeTap<DETAIL, T, D> extends Tap<T, D> {
  detail: DETAIL
}

export interface CollapseChange extends Tap<unknown, unknown> {
  detail: {
    value: number[]
  }
}

export interface DateTimePickerConfirm extends Tap<unknown, unknown> {
  detail: {
    value: string
  }
}
