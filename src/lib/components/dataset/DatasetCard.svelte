<script lang="ts">
  import type { Dataset } from "../../../../bindings";
  import TauriService from "../../../services/tauri-service";
  import { Link, navigate } from "svelte-routing";
  import RefreshIcon from "../icons/RefreshIcon.svelte";

  export let dataset: Dataset;
  let imageElement: Optional<HTMLImageElement> = undefined;

  async function getThumbnail(e: MouseEvent) {
    e.preventDefault();
    imageElement!.style.opacity = "0";

    setTimeout(async () => {
      dataset.thumbnail = await TauriService.getDatasetThumbnail(dataset.name);
      imageElement!.style.opacity = "1";
    }, 150);
  }
</script>

<div class="card w-64 min-h-40 bg-base-100 shadow-xl border-primary border-t-2">
  <div class="card-body items-center">
    <div class="relative w-48 bg-gray-400 rounded-sm">
      <img
        alt="Dataset Thumbnail"
        bind:this={imageElement}
        class="object-cover rounded-sm transition-opacity"
        src={`data:image/jpeg;base64,${dataset.thumbnail}`} />
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
    <p class="text-center">{dataset.data_amount} Images</p>
    <Link
      class="btn btn-primary btn-sm min-h-0 h-fit font-bold text-white py-2.5 mt-2 w-full"
      to={`/preprocess/${dataset.name}`}>
      Select Dataset
    </Link>
  </div>
</div>
