<script lang="ts">
  import DataLabelGrid from "../../lib/components/dataset/DataLabelGrid.svelte";
  import type { Dataset } from "../../../bindings";
  import TauriService from "../../services/tauri-service";
  import { Link } from "svelte-routing";

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
      <div class="card w-64 min-h-80 bg-base-100 shadow-xl sticky top-0 border-primary border-t-2">
        <div class="card-title pb-0 py-4 flex flex-col items-center justify-between">
          <img
            alt="dataset"
            class="w-32 h-32 rounded-full"
            src="/preprocess/dataset.png" />
          <h2 class="text-xl font-bold">{name}</h2>
          <hr class="my-2 w-full border-gray-200" />
        </div>
        <div class="card-body pt-2 flex flex-col h-full items-start justify-start">
          <div class="flex flex-col items-start flex-1">
            <div class="text-center">Total Types: {dataset.label_amount}</div>
            <div class="text-center">Total Images: {dataset.data_amount}</div>
            <div class="text-center">Processed Images: 0/{dataset.data_amount}</div>
          </div>
          <button class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"> Preprocess Dataset </button>
        </div>
      </div>
    </div>
    <DataLabelGrid {name} />
  </div>
</div>
