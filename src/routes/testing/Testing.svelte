<script lang="ts">
  import Template from "../../lib/components/Template.svelte";
  import ModelService from "../../services/model-service";
  import { scale } from "svelte/transition";
  import ImageCard from "../../lib/components/testing/ImageCard.svelte";
  import type { ModelPrediction } from "../../../bindings";
  import ImageUploadCard from "../../lib/components/testing/ImageUploadCard.svelte";
  import base64Converter from "../../lib/utils/base64-converter";
  import NProgress from "nprogress";

  export let name: string = "";

  let modelPrediction: Nullable<ModelPrediction> = null;
  let file: Nullable<File> = null;
  let isLoading = false;

  const getModel = async () => {
    NProgress.start();
    const model = await ModelService.getModel(name);
    NProgress.done();
    return model;
  };

  const predict = async () => {
    isLoading = true;
    NProgress.start();
    const base64 = (await base64Converter(file!)).split(",")[1];
    modelPrediction = await ModelService.predict(name, base64);
    isLoading = false;
    NProgress.done();
  };

  const rePredict = () => {
    modelPrediction = null;
    file = null;
  };

  $: inputImageUrl = file ? URL.createObjectURL(file) : null;
  $: isDisabled = !file || isLoading;
</script>

<Template
  description="Test Model"
  title="Test Model">
  <div class="w-full flex flex-row justify-start items-start gap-10 p-4">
    <div>
      <div class="card w-64 min-h-80 bg-base-100 shadow-xl sticky top-0 border-primary border-t-2">
        <div class="card-title pb-0 py-4 flex flex-col items-center justify-between">
          <img
            alt="dataset"
            class="w-32 h-32 rounded-full"
            src="/testing/model.png" />
          <h2 class="text-xl font-bold">{name}</h2>
          <hr class="my-2 w-full border-gray-200" />
        </div>
        <div class="card-body pt-2 flex flex-col h-full items-start justify-start">
          {#await getModel() then model}
            <div class="flex flex-col items-start flex-1">
              <div class="text-center">Trained Dataset: {model.model_specification.dataset_name}</div>
              <div class="text-center w-full">Accuracy: {(model.model_specification.accuracy * 100).toFixed(2)} %</div>
            </div>
          {/await}
          {#if isLoading}
            <button
              class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
              disabled>
              Loading...
            </button>
          {:else if modelPrediction}
            <button
              on:click={rePredict}
              class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full">Re-Predict</button>
          {:else}
            <button
              disabled={isDisabled}
              on:click={predict}
              class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full">Predict</button>
          {/if}
        </div>
      </div>
    </div>
    {#if !modelPrediction}
      <div class="w-full h-full flex flex-col items-center gap-10 me-[16%]">
        <ImageUploadCard bind:file />
      </div>
    {:else}
      <div class="w-full h-full flex flex-col items-center justify-center gap-10 me-[16%]">
        <div class="w-full h-full flex flex-row items-center justify-center gap-10">
          <ImageCard
            imageUrl={inputImageUrl ?? ""}
            title="Input Image" />
          <ImageCard
            imageUrl="data:image/jpeg;base64,{modelPrediction?.image_result}"
            title="Output Image" />
        </div>
        <div class="w-full h-full flex flex-row items-center justify-center gap-10">
          <div
            class="card w-[35rem] min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
            in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
            <div class="card-title flex flex-col justify-center items-center w-full py-2 gap-0">
              <h2 class="text-xl font-bold">Prediction</h2>
              <hr class="my-2 w-full border-gray-200" />
            </div>
            <div class="card-body items-center">{modelPrediction?.class}</div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</Template>
