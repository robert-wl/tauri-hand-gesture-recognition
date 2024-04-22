import BaseService from "./base-service";
import type { Model, ModelHyperparameter } from "../../bindings";

export default class ModelService extends BaseService {
  public static async trainModel(dataName: string, modelName: string, hyperparameter: ModelHyperparameter) {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.train(dataName, modelName, hyperparameter));
  }

  public static async predict(modelName: string, base64: string) {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.predict(modelName, base64));
  }

  public static async getAllModel(): Promise<Model[]> {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.get_all());
  }

  public static async getModel(modelName: string): Promise<Model> {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.get(modelName));
  }

  public static async removeModel(modelName: string) {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.remove(modelName));
  }
}
