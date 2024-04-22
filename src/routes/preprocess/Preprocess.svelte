<script lang="ts">
  import type { Dataset } from "../../../bindings";
  import DatasetService from "../../services/dataset-service";
  import DatasetLabelCard from "../../lib/components/preprocess/DatasetLabelCard.svelte";
  import Template from "../../lib/components/Template.svelte";
  import NProgress from "nprogress";
  import { onMount } from "svelte";
  import Loading from "../../lib/components/Loading.svelte";

  export let name: string = "";

  let isLoading = false;
  let dataset: Dataset = {
    name: "",
    labels: [],
  };

  const getDataset = async () => {
    NProgress.start();
    dataset = await DatasetService.getDataset(name);
    NProgress.done();
  };

  const preprocessDataset = async () => {
    NProgress.start();
    isLoading = true;

    dataset.labels = dataset.labels.map((label) => ({
      ...label,
      is_preprocessed: false,
    }));

    await DatasetService.preprocessDataset(name);

    isLoading = false;
    NProgress.done();
  };

  $: dataLength = dataset.labels.reduce((acc, val) => acc + val.data.length, 0);

  onMount(() => {
    getDataset();
  });
</script>

<Template
  description="Detect the landmarks in the images and collect the data for each image."
  title="Preprocess">
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
            <div class="text-center">Total Types: {dataset.labels.length}</div>
            <div class="text-center">Total Images: {dataLength}</div>
            <div class="text-center">Processed Images: 0/{dataLength}</div>
          </div>
          {#if isLoading}
            <button
              class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
              disabled>
              Loading...
            </button>
          {:else}
            <button
              class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
              on:click={preprocessDataset}>
              Preprocess Dataset
            </button>
          {/if}
        </div>
      </div>
    </div>
    {#if dataset.labels.length === 0}
      <div class="w-full h-full flex flex-col justify-center items-center gap-10">
        <Loading centered={false} />
      </div>
    {:else}
      <div class="w-full h-full grid 2xl:grid-cols-6 grid-cols-4 gap-10">
        {#each dataset.labels as label}
          <DatasetLabelCard
            datasetName={name}
            datasetLabel={label} />
        {/each}
      </div>
    {/if}
  </div>
</Template>
