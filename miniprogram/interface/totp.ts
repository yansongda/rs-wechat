interface ITotpItem extends ITotpItemResponse {
  isTouchMove: boolean,
}

interface ITotpDetailRequest extends IRequestQuery {
  id: number,
}

interface ITotpUpdateRequest extends IRequestData{
  issuer: string,
  username: string,
}

interface ITotpUpdateCreateRequest extends IRequestData{
  uri: string
}

interface ITotpItemResponse {
  id: number,
  issuer: string,
  username: string,
  code: string,
}

interface ITotpResponse {
}
