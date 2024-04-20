import BaseService from "./base-service";

export default class ModelService extends BaseService {
  private static preprocess_progress: any;

  public static async trainModel(dataName: string, modelName: string, kernel: string) {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.train(dataName, modelName, kernel));
  }
}
