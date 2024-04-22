<script lang="ts">
  import type { TrainingDataset } from "../../../../bindings";
  import DatasetService from "../../../services/dataset-service";
  import { Link } from "svelte-routing";
  import RefreshIcon from "../icons/RefreshIcon.svelte";
  import { scale, fade } from "svelte/transition";

  export let dataset: TrainingDataset;
  let imageElement: Optional<HTMLImageElement> = undefined;
  let thumbnail: string = "";

  async function getThumbnail() {
    if (!thumbnail) {
      thumbnail = await DatasetService.getRandomProcessedDatasetImage(dataset.name);
      return;
    }
    imageElement!.style.opacity = "0";

    setTimeout(async () => {
      thumbnail = await DatasetService.getRandomProcessedDatasetImage(dataset.name);
      imageElement!.style.opacity = "1";
    }, 150);
  }

  getThumbnail();
</script>

<div
  class="card w-64 min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-title flex flex-col gap-0">
    {#if !thumbnail}
      <div class="flex items-center w-full justify-center h-48">
        <p class="text-center text-lg"></p>
      </div>
    {:else}
      <img
        in:fade={{ duration: 500 }}
        out:fade={{ duration: 0 }}
        bind:this={imageElement}
        alt="Dataset Thumbnail"
        class="object-cover w-full h-48 rounded-sm transition-opacity rounded-t-none border-b border-gray-200"
        src={`data:image/jpeg;base64,${thumbnail}`} />
    {/if}
    <button
      class="btn btn-sm px-1.5 btn-ghost hover:bg-transparent hover:scale-110 rounded-md shadow-none outline-none text-black absolute top-0 right-0"
      on:click={getThumbnail}>
      <RefreshIcon
        height="1.5em"
        width="1.5em" />
    </button>
  </div>
  <div class="card-body items-center pt-4">
    <h2 class="card-title text-xl text-center">{dataset.name}</h2>
    <hr class="my-2 w-full border-gray-200" />
    <p class="text-center">{dataset.feature_count} Features</p>
    <p class="text-center">{dataset.data_amount} Data</p>
    <Link
      class="btn btn-primary btn-sm min-h-0 h-fit font-bold text-white py-2.5 mt-2 w-full"
      to={`/training-dataset/${dataset.name}`}>
      Select Dataset
    </Link>
  </div>
</div>
