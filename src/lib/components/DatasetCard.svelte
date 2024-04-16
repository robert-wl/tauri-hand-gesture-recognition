<script lang="ts">

  import type { Dataset } from "../../../bindings";
  import { appDataDir } from "@tauri-apps/api/path";

  export let dataset: Dataset;

  function selectDataset(dataset: Dataset) {
    const a = async () => {
      const b = await appDataDir();
      console.log("DIR", b);
    };
    a();
    localStorage.setItem("selectedDataset", JSON.stringify(dataset));
  }


</script>

<div
  class="card w-64 min-h-40 bg-base-100 hover:bg-gray-50 shadow-xl cursor-pointer duration-300 transition-all hover:-translate-y-2">
  <div class="card-body items-center">
    <h2 class="card-title text-center">{dataset.name}</h2>
    <hr class="my-2 w-full border-gray-200" />
    <p class="text-center">Dataset Amount: {dataset.data_amount}</p>
    <button
      class="btn btn-primary text-white !h-5 !min-h-10 w-full mt-2"
      on:click={() => selectDataset(dataset)}
    >
      Select
    </button>
  </div>
</div>