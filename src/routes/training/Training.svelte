<script lang="ts">
  import DatasetService from "../../services/dataset-service";
  import ModelService from "../../services/model-service";
  import ClassificationReport from "../../lib/components/model/ClassificationReport.svelte";
  import SpecificationReport from "../../lib/components/model/SpecificationReport.svelte";
  import type { Model } from "../../../bindings";
  import ConfusionMatrixReport from "../../lib/components/model/ConfusionMatrixReport.svelte";
  import { scale } from "svelte/transition";

  export let name: string = "";

  let isLoading = false;
  let datasetName = "Pick one";
  let modelName: Nullable<string>;
  let modelKernel = "RBF";
  let model: Nullable<Model>;

  const getModelDataset = async () => {
    return await DatasetService.getTrainingDatasets();
  };

  const trainModel = async () => {
    isLoading = true;
    await ModelService.trainModel(datasetName, modelName, modelKernel.toLowerCase());
    model = await ModelService.getModel(modelName);
    isLoading = false;
  };

  const saveModel = async () => {
    model = null;
  };

  const retrainModel = async () => {
    await ModelService.removeModel(modelName);
    model = null;
  };

  $: isDisabled = datasetName === "Pick one" || !modelName;
</script>

<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4">
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-16">Train Model</h1>
    <p class="text-center text-lg max-w-[40rem]">
      Train a model using the dataset of your choice. You can select the dataset, model name, and kernel type.
    </p>
  </div>
  <div class="w-full flex flex-row justify-center items-center gap-10 p-4">
    {#if !model}
      <div
        in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}
        class="sticky top-0">
        <div class="card w-64 min-h-80 bg-base-100 shadow-xl sticky top-0 border-primary border-t-2">
          <div class="card-title pb-0 py-4 flex flex-col items-center justify-between">
            <img
              alt="dataset"
              class="w-32 h-32 rounded-full"
              src="/training/model.png" />
            <h2 class="text-xl font-bold">{name}</h2>
            <hr class="my-2 w-full border-gray-200" />
          </div>
          <div class="flex flex-col w-full justify-center items-center gap-4">
            <label class="form-control w-4/5 max-w-xs">
              <div class="label">
                <span class="label-text">Select dataset</span>
              </div>
              <select
                bind:value={datasetName}
                class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
                <option
                  disabled
                  selected>Pick one</option>
                {#await getModelDataset() then dataset}
                  {#each dataset as d}
                    <option>{d.name}</option>
                  {/each}
                {/await}
              </select>
            </label>
            <label class="form-control w-4/5 max-w-xs">
              <div class="label">
                <span class="label-text">Model Name</span>
              </div>
              <input
                bind:value={modelName}
                class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
                placeholder="Jumbotron 3000"
                type="text" />
            </label>
            <label class="form-control w-4/5 max-w-xs">
              <div class="label">
                <span class="label-text">Select kernel</span>
              </div>
              <select
                bind:value={modelKernel}
                class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
                <option selected>RBF</option>
                <option>Linear</option>
                <option>Polynomial</option>
                <option>Sigmoid</option>
              </select>
            </label>
          </div>
          <div class="card-body pt-2 flex flex-col h-full items-start justify-start">
            <div class="flex flex-col items-start flex-1"></div>
            {#if isLoading}
              <button
                class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
                disabled>
                Loading...
              </button>
            {:else}
              <button
                on:click={trainModel}
                disabled={isDisabled}
                class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full">
                Train Model
              </button>
            {/if}
          </div>
        </div>
      </div>
    {:else}
      <div class="w-full h-full flex flex-row justify-center gap-10">
        <SpecificationReport
          {retrainModel}
          {saveModel}
          {model} />
        <ConfusionMatrixReport {model} />
        <ClassificationReport {model} />
      </div>
    {/if}
  </div>
</div>
