<script lang="ts">
  import type { Label, ProgressPayload } from "../../../../bindings";
  import DatasetService from "../../../services/dataset-service";
  import ProgressBar from "../ProgressBar.svelte";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import RefreshIcon from "../icons/RefreshIcon.svelte";
  import ImageIcon from "../icons/ImageIcon.svelte";
  import { scale, fade } from "svelte/transition";

  export let datasetName: string;
  export let datasetLabel: Label;

  let chosenImage: string = "";
  let progress: number = 0;

  const getRawImage = async (image: string) => {
    return await DatasetService.getRawImage(datasetName, datasetLabel.name, image);
  };

  const getProcessedImage = async (image: string) => {
    return await DatasetService.getProcessedImage(datasetName, datasetLabel.name, image);
  };

  const initListener = async () => {
    await listen(`progress_${datasetLabel.name}`, (event: TauriEvent<ProgressPayload>) => {
      progress = (event.payload.current_amount / event.payload.total_amount) * 100;

      if (progress >= 100) {
        datasetLabel.is_preprocessed = true;
      }
    });
  };

  const randomizeImage = async () => {
    chosenImage = datasetLabel.data[Math.floor(Math.random() * datasetLabel.data.length)];
  };

  onMount(() => {
    initListener();
    randomizeImage();
  });

  $: thumbnail = datasetLabel && datasetLabel.is_preprocessed ? getProcessedImage(chosenImage) : getRawImage(chosenImage);
  $: realProgress = datasetLabel && datasetLabel.is_preprocessed ? 100 : progress;
</script>

<div
  class="card min-h-40 bg-base-100 shadow-xl rounded-t-none"
  in:scale|global={{ duration: 100, opacity: 0.5, start: 0.5 }}>
  <div class="card-title flex flex-col gap-0">
    <ProgressBar progress={realProgress} />
    {#await thumbnail}
      <div class="flex items-center w-full justify-center h-48">
        <p class="text-center text-lg"></p>
      </div>
    {:then t}
      <img
        in:fade={{ duration: 500 }}
        out:fade={{ duration: 0 }}
        alt="Dataset Thumbnail"
        class="object-cover w-full h-48 rounded-sm transition-opacity rounded-t-none"
        src={`data:image/jpeg;base64,${t}`} />
    {/await}
    <button
      class="btn btn-sm px-1.5 btn-ghost hover:bg-transparent hover:scale-110 rounded-md shadow-none outline-none text-black absolute top-0 right-0"
      on:click={randomizeImage}>
      <RefreshIcon
        height="1.5em"
        width="1.5em" />
    </button>
  </div>
  <div class="card-body p-4 items-start">
    <h2 class="card-title text-xl text-center">{datasetLabel.name}</h2>
    <hr class="my-2 w-full border-gray-200" />
    <p class="flex flex-row items-center gap-2">
      <ImageIcon />
      <span>
        {datasetLabel.data.length} Images
      </span>
    </p>
  </div>
</div>
