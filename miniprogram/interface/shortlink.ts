interface IShortlinkCreateRequest extends IRequestData {
  link: string,
}

interface IShortlinkCreateResponse {
  link: string,
  shortlink: string,
}
