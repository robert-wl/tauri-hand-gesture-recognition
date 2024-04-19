import BaseService from "./base-service";

export default class EventService extends BaseService {
  private static preprocess_progress: any;

  public static async listenForDatasetChanges() {
    const ipc = await this.getTauRPCProxy();
    if (!this.preprocess_progress) {
      this.preprocess_progress = ipc.dataset.preprocess_progress;
    }
    return this.preprocess_progress;
  }
}
