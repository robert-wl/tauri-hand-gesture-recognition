import { createTauRPCProxy } from "../../bindings";

export default class BaseService {
  private static ipc: Awaited<ReturnType<typeof createTauRPCProxy>>;
  protected static async getTauRPCProxy() {
    if (!this.ipc) {
      this.ipc = await createTauRPCProxy();
    }
    return this.ipc;
  }
}
