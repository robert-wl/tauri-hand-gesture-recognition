<script lang="ts">
  import type { Label, ProgressPayload } from "../../../../bindings";
  import TauriService from "../../../services/tauri-service";
  import ProgressBar from "../ProgressBar.svelte";
  import { onDestroy, onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  export let datasetName: string;
  export let datasetLabel: Label;

  let thumbnail: string = "";
  let progress: number = 0;

  const getThumbnail = async () => {
    thumbnail = await TauriService.getRandomDatasetImage(`${datasetName}/${datasetLabel.name}`);
  };

  const initListener = async () => {
    listen(`progress_${datasetLabel.name}`, (event: TauriEvent<ProgressPayload>) => {
      console.log(event.payload);
      progress = (event.payload.current_amount / event.payload.total_amount) * 100;
    });
  };

  onMount(() => {
    getThumbnail();
    initListener();
  });
</script>

<button on:click>
  <div class="card min-h-40 bg-base-100 hover:bg-gray-50 shadow-xl rounded-t-none cursor-pointer duration-300 transition-all hover:-translate-y-2">
    <div class="card-title flex flex-col gap-0">
      <ProgressBar progress={datasetLabel.is_preprocessed ? 100 : progress} />
      <img
        alt="Dataset Thumbnail"
        class="object-cover rounded-sm transition-opacity rounded-t-none"
        src={`data:image/jpeg;base64,${thumbnail}`} />
    </div>
    <div class="card-body p-4 items-center">
      <h2 class="card-title text-xl text-center pb-2">{datasetLabel.name}</h2>
      <p class="text-center">{datasetLabel.data.length} Images</p>
    </div>
  </div>
</button>
