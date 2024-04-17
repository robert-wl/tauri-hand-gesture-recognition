<script lang="ts">
  import DataLabelGrid from "../../lib/components/dataset/DataLabelGrid.svelte";
  import type { Dataset } from "../../../bindings";
  import TauriService from "../../services/tauri-service";

  export let name: string = "test";
  let dataset: Dataset = {
    name: "",
    thumbnail: "",
    data_amount: -1,
    label_amount: -1,
  };

  const getDataset = async () => {
    dataset = await TauriService.getDataset(name);
  };
  getDataset();
</script>

<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4">
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-16">Datasets</h1>
    <p class="text-center text-lg max-w-[40rem]">Choose your dataset from the list of available datasets.</p>
  </div>
  <div class="w-full flex flex-row justify-start items-start gap-10 p-4">
    <div>
      <div class="card w-64 min-h-80 bg-base-100 shadow-xl sticky top-0">
        <div class="card-body flex flex-col h-full items-center justify-start sticky top-0">
          <h2 class="card-title text-xl text-center pb-2">{name}</h2>
          <hr class="my-2 w-full border-gray-200" />
          <p class="text-center">Total Types: {dataset.label_amount}</p>
          <p class="text-center">Total Images: {dataset.data_amount}</p>
          <p class="text-center">Processed Images: 0/{dataset.data_amount}</p>
          <button class="btn btn-accent text-white mt-2">Preprocess Dataset</button>
        </div>
      </div>
    </div>
    <DataLabelGrid {name} />
  </div>
</div>
