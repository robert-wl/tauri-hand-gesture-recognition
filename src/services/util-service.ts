import BaseService from "./base-service";

export default class UtilService extends BaseService {
  public static async openDatasetDirectory() {
    return await this.getTauRPCProxy().then((ipc) => ipc.util.open_directory());
  }
}
