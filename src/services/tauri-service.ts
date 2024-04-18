import { createTauRPCProxy, type Dataset, type Label, type GeneralDataset } from "../../bindings";

export default class TauriService {
  private static ipc: Awaited<ReturnType<typeof createTauRPCProxy>>;

  public static async getDatasets(): Promise<GeneralDataset[]> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_all();
  }

  public static async getCurrentDirectory(): Promise<string> {
    const ipc = await this.getTauRPCProxy();
    return ipc.util.get_current_dir();
  }

  public static async getDatasetThumbnail(dataName: string): Promise<string> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_random_thumbnail(dataName);
  }

  public static async getDataset(dataName: string): Promise<Dataset> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get(dataName);
  }

  public static async getDatasetLabel(dataName: string): Promise<Label[]> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_labels(dataName);
  }

  public static async openDatasetDirectory() {
    const ipc = await this.getTauRPCProxy();
    return ipc.util.open_directory();
  }

  public static async getDatasetData(dataName: string, label: string) {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_data(dataName, label);
  }

  private static async getTauRPCProxy() {
    if (!this.ipc) {
      this.ipc = await createTauRPCProxy();
    }
    return this.ipc;
  }
}
