<script lang="ts">
  import type { DatasetLabel } from "../../../../bindings";
  import DataPreviewModal from "./DataPreviewModal.svelte";

  export let datasetLabel: DatasetLabel;
  let dialog: HTMLDialogElement;
  let progress = 0;

  const stateColor = {
    loading: "progress-warning",
    processed: "progress-success",
    unprocessed: "progress-base-100",
  };

  let interval: ReturnType<typeof setInterval>;
</script>

<button on:click={() => dialog.showModal()}>
  <div class="card min-h-40 bg-base-100 hover:bg-gray-50 shadow-xl rounded-t-none cursor-pointer duration-300 transition-all hover:-translate-y-2">
    <div class="card-title flex flex-col gap-0">
      <progress
        class={`progress w-full bg-base-500 h-1 ${progress === 100 ? stateColor.processed : stateColor.loading}`}
        max="100"
        value={progress}></progress>
      <img
        alt="Dataset Thumbnail"
        class="object-cover rounded-sm transition-opacity rounded-t-none"
        src={`data:image/jpeg;base64,${datasetLabel.thumbnail}`} />
    </div>
    <div class="card-body p-4 items-center">
      <h2 class="card-title text-xl text-center pb-2">{datasetLabel.name}</h2>
      <p class="text-center">{datasetLabel.data_amount} Images</p>
    </div>
  </div>
  <DataPreviewModal bind:dialog />
</button>
