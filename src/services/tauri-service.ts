import { createTauRPCProxy, type Dataset, type DatasetLabel } from "../../bindings";

export default class TauriService {
  private static ipc: Awaited<ReturnType<typeof createTauRPCProxy>>;

  public static async getDatasets(): Promise<Dataset[]> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_datasets();
  }

  public static async getCurrentDirectory(): Promise<string> {
    const ipc = await this.getTauRPCProxy();
    return ipc.util.get_current_dir();
  }

  public static async getDatasetThumbnail(dataName: string): Promise<string> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_dataset_thumbnail(dataName);
  }

  public static async getDataset(dataName: string): Promise<Dataset> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_dataset(dataName);
  }

  public static async getDatasetLabel(dataName: string): Promise<DatasetLabel[]> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_dataset_label_data(dataName);
  }

  public static async openDatasetDirectory() {
    const ipc = await this.getTauRPCProxy();
    return ipc.util.open_directory();
  }

  private static async getTauRPCProxy() {
    if (!this.ipc) {
      this.ipc = await createTauRPCProxy();
    }
    return this.ipc;
  }
}
