import BaseService from "./base-service";
import type { Model } from "../../bindings";

export default class ModelService extends BaseService {
  public static async trainModel(dataName: string, modelName: string, kernel: string) {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.train(dataName, modelName, kernel));
  }

  public static async getModel(modelName: string): Promise<Model> {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.get(modelName));
  }

  public static async removeModel(modelName: string) {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.remove(modelName));
  }
}
