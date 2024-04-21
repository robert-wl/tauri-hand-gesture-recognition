<script lang="ts">
  import { scale, fade } from "svelte/transition";
  import ModelService from "../../../services/model-service";
  import type { Model, ModelHyperparameter } from "../../../../bindings";

  export let name: string;
  export let trainModel: (modelName: string, hyperparameter: ModelHyperparameter) => void;

  let hyperparameters: ModelHyperparameter = {
    C: "1.0",
    gamma: "scale",
    degree: "3",
    kernel: "rbf",
  };

  let isLoading = false;
  let model: Nullable<Model>;
  let modelName: Nullable<string>;

  const validateHyperparameters = (params) => {
    if (isNaN(Number(params.C)) || Number(params.C) <= 0) {
      return false;
    }

    const degree = Number(params.degree);
    if (params.kernel === "poly" && (isNaN(degree) || degree <= 0 || !Number.isInteger(degree))) {
      return false;
    }

    const gamma = Number(params.gamma);
    if (params.gamma !== "auto" && params.gamma !== "scale" && (isNaN(gamma) || gamma <= 0)) {
      return false;
    }
    return true;
  };

  const handleTrain = async () => {
    isLoading = true;
    await trainModel(modelName, hyperparameters);
    isLoading = false;
  };

  $: isDisabled = name === "Pick one" || !modelName || !validateHyperparameters(hyperparameters);
  $: showPoly = hyperparameters.kernel === "poly";
</script>

<div
  class="sticky top-0 flex flex-col gap-4"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card w-[35rem] bg-base-100 shadow-xl sticky top-0 border-primary border-t-2">
    <div class="card-title flex flex-row justify-center items-center w-full gap-0">
      <div class="py-4 flex h-56 flex-col w-2/5 border-r border-gray-200 items-center justify-center">
        <img
          alt="dataset"
          class="w-32 h-32 rounded-full"
          src="/training/model.png" />
        <h2 class="text-xl font-bold">{name}</h2>
      </div>
      <div class="flex flex-col w-3/5 border-r border-gray-200 items-center justify-start">
        <div class="form-control w-4/5 max-w-xs">
          <div class="label">
            <span class="label-text">Model Name</span>
          </div>
          <input
            bind:value={modelName}
            class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
            placeholder="Jumbotron 3000"
            type="text" />
        </div>
        <div class="form-control w-4/5 max-w-xs">
          <div class="label">
            <span class="label-text">Select kernel</span>
          </div>
          <select
            bind:value={hyperparameters.kernel}
            class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
            <option
              selected
              value="rbf">RBF</option>
            <option value="linear">Linear</option>
            <option value="poly">Polynomial</option>
            <option value="sigmoid">Sigmoid</option>
          </select>
        </div>
      </div>
    </div>
  </div>
  {#if modelName}
    <div
      in:scale={{ duration: 200, opacity: 0.5, start: 0.5 }}
      out:fade={{ duration: 200, opacity: 0.5 }}
      class="card w-[35rem] bg-base-100 shadow-xl sticky top-0 border-primary border-t-2">
      <div class="card-title flex flex-col justify-center items-center w-full py-2 gap-0">
        <h2 class="text-xl font-bold">Hyperparameter Tuning</h2>
        <hr class="my-2 w-full border-gray-200" />
      </div>
      <div class="card-body pt-2 flex flex-col h-full items-start justify-start">
        <div class="flex flex-col w-full items-center flex-1">
          <div class="form-control w-4/5 max-w-xs">
            <div class="label">
              <span class="label-text">C</span>
            </div>
            <input
              bind:value={hyperparameters.C}
              class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
              placeholder="Positive float"
              type="text" />
          </div>
          <div class="form-control w-4/5 max-w-xs">
            <div class="label">
              <span class="label-text">Gamma</span>
            </div>
            <input
              bind:value={hyperparameters.gamma}
              class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
              placeholder="Auto, scale or a positive float"
              type="text" />
          </div>
          {#if showPoly}
            <div
              in:scale={{ duration: 200, opacity: 0.5, start: 0.5 }}
              out:fade={{ duration: 200, opacity: 0.5 }}
              class="form-control w-4/5 max-w-xs">
              <div class="label">
                <span class="label-text">Degree</span>
              </div>
              <input
                bind:value={hyperparameters.degree}
                class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
                placeholder="Positive integer"
                type="text" />
            </div>
          {/if}
          {#if isLoading}
            <button
              class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-5 w-2/3"
              disabled>
              Loading...
            </button>
          {:else}
            <button
              on:click={handleTrain}
              disabled={isDisabled}
              class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-5 w-2/3">
              Train Model
            </button>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>
