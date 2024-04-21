<script lang="ts">
  import type { GeneralDataset } from "../../../../bindings";
  import DatasetService from "../../../services/dataset-service";
  import { Link } from "svelte-routing";
  import RefreshIcon from "../icons/RefreshIcon.svelte";
  import { scale } from "svelte/transition";

  export let dataset: GeneralDataset;
  let imageElement: Optional<HTMLImageElement> = undefined;
  let thumbnail: string = "";

  async function getThumbnail() {
    if (!thumbnail) {
      thumbnail = await DatasetService.getRandomDatasetImage(dataset.name);
      return;
    }
    imageElement!.style.opacity = "0";

    setTimeout(async () => {
      thumbnail = await DatasetService.getRandomDatasetImage(dataset.name);
      imageElement!.style.opacity = "1";
    }, 150);
  }

  getThumbnail();
</script>

<div
  class="card w-64 min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-body items-center">
    <div class="relative w-48 bg-gray-400 rounded-sm">
      {#if !thumbnail}
        <div class="absolute inset-0 flex items-center justify-center">
          <div class="spinner spinner-primary"></div>
        </div>
      {:else}
        <img
          alt="Dataset Thumbnail"
          bind:this={imageElement}
          class="object-cover rounded-sm transition-opacity"
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
    <hr class="my-2 w-full border-gray-200" />
    <h2 class="card-title text-xl text-center pb-2">{dataset.name}</h2>
    <p class="text-center">{dataset.label_amount} Labels</p>
    <p class="text-center">{dataset.data_amount} Images</p>
    <Link
      class="btn btn-primary btn-sm min-h-0 h-fit font-bold text-white py-2.5 mt-2 w-full"
      to={`/preprocess/${dataset.name}`}>
      Select Dataset
    </Link>
  </div>
</div>
