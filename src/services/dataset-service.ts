import { type Dataset, type GeneralDataset, type TrainingDataset } from "../../bindings";
import BaseService from "./base-service";

export default class DatasetService extends BaseService {
  public static async getPreprocessDatasets(): Promise<GeneralDataset[]> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_all());
  }

  public static async getTrainingDatasets(): Promise<TrainingDataset[]> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_all_training_dataset());
  }

  public static async getRandomDatasetImage(dataName: string): Promise<string> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_random_image(dataName));
  }

  public static async getRandomProcessedDatasetImage(dataName: string): Promise<string> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_random_processed_image(dataName));
  }

  public static async getRawDatasetImage(dataName: string, labelName: string, imageName: string): Promise<string> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_image(dataName, labelName, imageName));
  }

  public static async getProcessedDatasetImage(dataName: string, labelName: string, imageName: string): Promise<Nullable<string>> {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_processed_image(dataName, labelName, imageName));
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

  public static async getPreprocessedGraph(dataName: string) {
    return await this.getTauRPCProxy().then((ipc) => ipc.dataset.get_processed_graphs(dataName));
  }
}
