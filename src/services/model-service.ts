import BaseService from "./base-service";
import type { Model, Hyperparameters } from "../../bindings";

export default class ModelService extends BaseService {
  public static async trainModel(dataName: string, modelName: string, algorithm: string, hyperparameter: Hyperparameters) {
    return await this.getTauRPCProxy().then((ipc) => ipc.model.train(dataName, modelName, algorithm, hyperparameter));
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
