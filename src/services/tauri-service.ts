import { createTauRPCProxy, type Dataset, type Label, type GeneralDataset } from "../../bindings";
import BaseService from "./base-service";

export default class TauriService extends BaseService {
  public static async getDatasets(): Promise<GeneralDataset[]> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_all();
  }

  public static async getRandomDatasetImage(dataName: string): Promise<string> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_random_image(dataName);
  }

  public static async getRawDatasetImage(dataName: string, labelName: string, imageName: string): Promise<string> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_image(dataName, labelName, imageName);
  }

  public static async getProcessedDatasetImage(dataName: string, labelName: string, imageName: string): Promise<Nullable<string>> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_processed_image(dataName, labelName, imageName);
  }

  public static async getDataset(dataName: string): Promise<Dataset> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get(dataName);
  }

  public static async getRawImage(dataName: string, labelName: string, imageName: string): Promise<string> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_image(dataName, labelName, imageName);
  }

  public static async getProcessedImage(dataName: string, labelName: string, imageName: string): Promise<Nullable<string>> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_processed_image(dataName, labelName, imageName);
  }

  public static async preprocessDataset(dataName: string) {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.preprocess(dataName);
  }

  public static async openDatasetDirectory() {
    const ipc = await this.getTauRPCProxy();
    return ipc.util.open_directory();
  }
}
