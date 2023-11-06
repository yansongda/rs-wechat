interface IShortlinkCreateRequest extends IRequestData {
  link: string,
}

interface IShortlinkCreateResponse {
  link: string,
  short: string,
}
