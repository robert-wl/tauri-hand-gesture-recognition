<script lang="ts">
  import type { Dataset } from "../../../../bindings";
  import TauriService from "../../../services/tauri-service";

  export let dataset: Dataset;
  let imageElement: Optional<HTMLImageElement> = undefined;

  async function getThumbnail() {
    imageElement!.style.opacity = "0";

    setTimeout(async () => {
      dataset.thumbnail = await TauriService.getDatasetThumbnail(dataset.name);
      imageElement!.style.opacity = "1";
    }, 150);
  }

  function selectDataset(dataset: Dataset) {
    localStorage.setItem("selectedDataset", JSON.stringify(dataset));
  }
</script>

<button
  on:click={() => selectDataset(dataset)}
  on:mouseenter={getThumbnail}>
  <div class="card w-64 min-h-40 bg-base-100 hover:bg-gray-50 shadow-xl cursor-pointer duration-300 transition-all hover:-translate-y-2">
    <div class="card-body items-center">
      <h2 class="card-title text-xl text-center pb-2">{dataset.name}</h2>
      <div class="w-48 bg-gray-400 rounded-md">
        <img
          alt="Dataset Thumbnail"
          bind:this={imageElement}
          class="object-cover rounded-md transition-opacity"
          src={`data:image/jpeg;base64,${dataset.thumbnail}`} />
      </div>
      <hr class="my-2 w-full border-gray-200" />
      <p class="text-center">{dataset.data_amount} Images</p>
    </div>
  </div>
</button>
