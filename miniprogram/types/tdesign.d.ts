import { Tap } from "./wechat";

export interface TabbarTap<T, D> extends Tap<T, D> {
  detail: {
    value: String
  }
}
