<script lang="ts">
  import { scale, fade } from "svelte/transition";
  import type { Hyperparameters, SVMHyperparameter, KNNHyperparameter, LRHyperparameter } from "../../../../bindings";
  import SVMHyperparameters from "./SVMHyperparameters.svelte";
  import KNNHyperparameters from "./KNNHyperparameters.svelte";
  import LRHyperparameters from "./LRHyperparameters.svelte";

  export let name: string;
  export let trainModel: (modelName: string, algorithm: string, hyperparameter: Hyperparameters) => Promise<void>;

  let algorithm: string;

  let hyperparameters = {
    Svm: {
      C: "1.0",
      gamma: "scale",
      degree: "3",
      kernel: "rbf",
    },
    Knn: {
      metric: "minkowski",
      algorithm: "auto",
      n_neighbors: "5",
      weights: "uniform",
    },
    Lr: {
      penalty: "l2",
      C: "1.0",
      solver: "lbfgs",
      max_iter: "100",
    },
  };

  let isLoading = false;
  let modelName: Nullable<string>;

  const validateSVMHyperparameters = (params: SVMHyperparameter) => {
    if (isNaN(Number(params.C)) || Number(params.C) <= 0) {
      return false;
    }

    const degree = Number(params.degree);
    if (params.kernel === "poly" && (isNaN(degree) || degree <= 0 || !Number.isInteger(degree))) {
      return false;
    }

    const gamma = Number(params.gamma);

    return !(params.gamma !== "auto" && params.gamma !== "scale" && (isNaN(gamma) || gamma <= 0));
  };

  const validateKNNHyperparameters = (params: KNNHyperparameter) => {
    if (isNaN(Number(params.n_neighbors)) || Number(params.n_neighbors) <= 0) {
      return false;
    }

    return params.metric === "minkowski" && ["minkowski", "euclidean", "manhattan", "chebyshev"].includes(params.metric);
  };

  const validateLRHyperparameters = (params: LRHyperparameter) => {
    if (isNaN(Number(params.C)) || Number(params.C) <= 0) {
      return false;
    }

    return (
      ["l2", "l1", "elasticnet", "none"].includes(params.penalty) &&
      ["lbfgs", "liblinear", "newton-cg", "newton-cholesky", "sag", "saga"].includes(params.solver)
    );
  };

  const validateHyperparameters = (params: Hyperparameters) => {
    switch (algorithm) {
      case "svm":
        return "Svm" in params ? validateSVMHyperparameters(params.Svm) : false;
      case "knn":
        return "Knn" in params ? validateKNNHyperparameters(params.Knn) : false;
      case "lr":
        return "Lr" in params ? validateLRHyperparameters(params.Lr) : false;
      default:
        return false;
    }
  };

  const handleTrain = async () => {
    isLoading = true;

    let hyperParam: Hyperparameters;
    switch (algorithm) {
      case "svm":
        hyperParam = { Svm: hyperparameters.Svm };
        break;
      case "knn":
        hyperParam = { Knn: hyperparameters.Knn };
        break;
      case "lr":
        hyperParam = { Lr: hyperparameters.Lr };
        break;
      default:
        isLoading = false;
        return;
    }

    await trainModel(modelName!, algorithm, hyperParam);
    isLoading = false;
  };

  $: isDisabled = name === "Pick one" || !modelName || !validateHyperparameters(hyperparameters);
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
            <span class="label-text">Model Algorithm</span>
          </div>
          <select
            bind:value={algorithm}
            class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
            <option
              selected
              value="svm">Support Vector Machine</option>
            <option value="lr">Logistic Regression</option>
            <option value="knn"> K-Nearest Neighbors</option>
          </select>
        </div>
      </div>
    </div>
  </div>
  {#if modelName}
    <div
      in:scale={{ duration: 200, opacity: 0.5, start: 0.5 }}
      out:fade={{ duration: 200 }}
      class="card w-[35rem] bg-base-100 shadow-xl sticky top-0 border-primary border-t-2">
      <div class="card-title flex flex-col justify-center items-center w-full py-2 gap-0">
        <h2 class="text-xl font-bold">Hyperparameter Tuning</h2>
        <hr class="my-2 w-full border-gray-200" />
      </div>
      {#if algorithm === "svm"}
        <SVMHyperparameters bind:hyperparameters={hyperparameters.Svm} />
      {:else if algorithm === "knn"}
        <KNNHyperparameters bind:hyperparameters={hyperparameters.Knn} />
      {:else if algorithm === "lr"}
        <LRHyperparameters bind:hyperparameters={hyperparameters.Lr} />
      {/if}
      <div class="flex flex-col items-center justify-center w-full">
        {#if isLoading}
          <button
            class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 my-5 w-1/2"
            disabled>
            Loading...
          </button>
        {:else}
          <button
            on:click={handleTrain}
            disabled={isDisabled}
            class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 my-5 w-1/2">
            Train Model
          </button>
        {/if}
      </div>
    </div>
  {/if}
</div>
