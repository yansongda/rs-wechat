## v1.2.0

### fixed

- fix(backend): 中文长度计算错误的问题(#54)

### chore

- chore(frontend): 由 `weui` 更换为 `tdesign-miniprogram`(#53)

## v1.1.10

### optimized

- optimize(frontend): 优化 totp 定时器更新机制(#46)

## v1.1.9

### fixed

- fix(backend): totp 独立更新周期不生效(#44)

## v1.1.8

### added

- feat(backend): totp 支持独立更新时间(#41)
- feat(frontend): totp 支持独立更新时间(#42)

## v1.1.7

### fixed

- fix(frontend): 锁屏前在 TOTP 页面锁屏后小程序 js 报异常(#35)
- fix(frontend): 二维码扫描出错时微信js报错(#36)

## v1.1.6

### optimized

- optimized(backend): 优化提示信息(#32)

## v1.1.5

### optimized

- optimized(frontend:submit): 优化提交中的 Promise 逻辑(#26)

### added

- feat(backend): 参数验证(#28)
- feat(backend): 支持 tracing(#29)

## v1.1.4

### added

- feat(frontend:core): 增加小程序升级提示功能(#25)

### optimized

- optimized(style): 消除 ts 的 any，增加 eslint 等检查(#23)

## v1.1.3

### fixed

- fixed(frontend:totp): totp 创建后未自动刷新列表(#15)
- fixed(frontend:totp): totp 列表多时，创建按钮会被覆盖(#18)

### chore

- chore: 请求报错：unable to get local issuer certificate(#20)

## v1.1.2

### optimized

- optimized(frontend:totp): totp 页面使用 weui-slideview 左滑功能提升性能(#11)
