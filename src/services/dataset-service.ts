import { type Dataset, type GeneralDataset, type ModelDataset } from "../../bindings";
import BaseService from "./base-service";

export default class DatasetService extends BaseService {
  public static async getDatasets(): Promise<GeneralDataset[]> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_all());
  }

  public static async getRandomDatasetImage(dataName: string): Promise<string> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_random_image(dataName));
  }

  public static async getRawDatasetImage(dataName: string, labelName: string, imageName: string): Promise<string> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_image(dataName, labelName, imageName));
  }

  public static async getProcessedDatasetImage(dataName: string, labelName: string, imageName: string): Promise<Nullable<string>> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_processed_image(dataName, labelName, imageName));
  }

  public static async getModelDataset(): Promise<ModelDataset[]> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_all_model_dataset());
  }

  public static async getDataset(dataName: string): Promise<Dataset> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get(dataName));
  }

  public static async getRawImage(dataName: string, labelName: string, imageName: string): Promise<string> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_image(dataName, labelName, imageName));
  }

  public static async getProcessedImage(dataName: string, labelName: string, imageName: string): Promise<Nullable<string>> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_processed_image(dataName, labelName, imageName));
  }

  public static async preprocessDataset(dataName: string) {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.preprocess(dataName));
  }

  public static async openDatasetDirectory() {
    return await this.getTauRPCProxy().then((ipc) => ipc.util.open_directory());
  }
}
